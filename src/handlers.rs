use crate::config::Config;
use crate::error::AppError;
use crate::hardware::estimate_ram_for_gguf;
use crate::llm::{run_chat, ChatContext};
use crate::state::{AppState, ChatMessage};
use axum::{
    body::Body,
    extract::State,
    response::{sse::{Event, Sse}, Html, IntoResponse, Response},
    Json,
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tokio_stream::wrappers::ReceiverStream;

static INDEX_HTML: &str = include_str!("templates/index.html");

pub async fn index_handler() -> Html<String> {
    let html = INDEX_HTML.replace("{{VERSION}}", env!("CARGO_PKG_VERSION"));
    Html(html)
}

pub async fn reset_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut hist = state.history.lock().await;
    let mut m = state.mode.lock().await;
    hist.clear();
    *m = "general".to_string();
    Json(serde_json::json!({"ok": true}))
}

pub async fn hw_handler(State(state): State<AppState>) -> impl IntoResponse {
    Json((*state.hw_info).clone())
}

pub async fn models_handler(State(state): State<AppState>) -> impl IntoResponse {
    let client = state.client.clone();
    let ollama_url = state.config.ollama.url.clone();
    let defaults = state.config.default_models();
    if let Ok(r) = client.get(format!("{}/api/tags", ollama_url)).send().await {
        if let Ok(json) = r.json::<serde_json::Value>().await {
            let models: Vec<String> = json["models"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
                .collect();
            return Json(serde_json::json!({
                "ollama_models": models,
                "default_models": defaults
            }))
            .into_response();
        }
    }
    Json(serde_json::json!({ "ollama_models": [], "default_models": defaults })).into_response()
}

async fn fetch_installed_models(client: &reqwest::Client, ollama_url: &str) -> Vec<String> {
    if let Ok(r) = client.get(format!("{}/api/tags", ollama_url)).send().await {
        if let Ok(json) = r.json::<serde_json::Value>().await {
            return json["models"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
                .collect();
        }
    }
    Vec::new()
}

#[derive(Deserialize)]
pub struct HfFilesRequest {
    pub repo: String,
}

#[derive(Serialize, Clone)]
pub struct GgufFileInfo {
    pub filename: String,
    pub size_mb: f64,
    pub estimated_ram_gb: f32,
    pub quant: String,
    pub compatible: bool,
    pub recommended: bool,
    pub ollama_id: String,
}

pub async fn hf_files_handler(
    State(state): State<AppState>,
    Json(payload): Json<HfFilesRequest>,
) -> impl IntoResponse {
    let repo = payload.repo.trim().to_string();
    if repo.is_empty() {
        return Json(serde_json::json!({"error": "ID repo vuoto", "files": []})).into_response();
    }
    let client = state.client.clone();
    let hw = state.hw_info.clone();
    let api_url = format!("https://huggingface.co/api/models/{}", repo);
    let resp = match client.get(&api_url).send().await {
        Ok(r) => r,
        Err(e) => {
            return Json(serde_json::json!({
                "error": format!("Errore connessione HF: {}", e),
                "files": []
            }))
            .into_response()
        }
    };
    let json: serde_json::Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => {
            return Json(serde_json::json!({
                "error": format!("Errore parsing HF: {}", e),
                "files": []
            }))
            .into_response()
        }
    };
    let siblings = json["siblings"].as_array().cloned().unwrap_or_default();
    let rec_quant = hw.recommended_quant.clone();
    let max_ram = hw.max_model_ram_gb;

    let mut files: Vec<GgufFileInfo> = siblings
        .iter()
        .filter_map(|s| {
            let fname = s["rfilename"].as_str()?;
            if !fname.to_lowercase().ends_with(".gguf") {
                return None;
            }
            let size_bytes = s["size"].as_f64().unwrap_or(0.0);
            let size_mb = size_bytes / 1_048_576.0;
            let estimated_ram = estimate_ram_for_gguf(fname);
            let compatible = estimated_ram <= max_ram;
            let quant = extract_quant_label(fname);
            let recommended = quant.to_uppercase() == rec_quant.to_uppercase();
            let ollama_id = format!("hf.co/{}:{}", repo, quant);
            Some(GgufFileInfo {
                filename: fname.to_string(),
                size_mb,
                estimated_ram_gb: estimated_ram,
                quant,
                compatible,
                recommended,
                ollama_id,
            })
        })
        .collect();

    files.sort_by(|a, b| {
        b.compatible
            .cmp(&a.compatible)
            .then(b.recommended.cmp(&a.recommended))
    });

    Json(serde_json::json!({
        "files": files,
        "recommended_quant": rec_quant,
        "max_model_ram_gb": max_ram,
    }))
    .into_response()
}

pub fn extract_quant_label(filename: &str) -> String {
    let f = filename.to_uppercase();
    let patterns = [
        "IQ1_S", "IQ1_M", "IQ2_XXS", "IQ2_XS", "IQ2_S", "IQ2_M",
        "IQ3_XXS", "IQ3_XS", "IQ3_S", "IQ3_M", "IQ4_XS", "IQ4_NL",
        "Q2_K", "Q3_K_S", "Q3_K_M", "Q3_K_L", "Q4_0", "Q4_1",
        "Q4_K_S", "Q4_K_M", "Q5_0", "Q5_1", "Q5_K_S", "Q5_K_M",
        "Q6_K", "Q8_0", "F16", "BF16",
    ];
    for pat in &patterns {
        if f.contains(pat) {
            return pat.to_string();
        }
    }
    if let Some(pos) = f.find('Q') {
        let sub = &f[pos..];
        let end = sub
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(sub.len());
        return sub[..end].to_string();
    }
    "UNKNOWN".to_string()
}

#[derive(Deserialize)]
pub struct ConnectRequest {
    pub source: String,
    pub model_id: String,
}

#[derive(Serialize)]
pub struct ConnectResponse {
    pub ok: bool,
    pub message: String,
    pub model_id: String,
}

async fn resolve_ollama_id(
    _config: &Config,
    client: &reqwest::Client,
    hw_max_ram: f32,
    hw_quant: &str,
    source: &str,
    model_id: &str,
) -> String {
    match source {
        "huggingface" => {
            if model_id.starts_with("hf.co/") {
                return model_id.to_string();
            }
            let api_url = format!("https://huggingface.co/api/models/{}", model_id);
            if let Ok(r) = client.get(&api_url).send().await {
                if let Ok(json) = r.json::<serde_json::Value>().await {
                    let siblings = json["siblings"].as_array().cloned().unwrap_or_default();
                    let mut best_q: Option<String> = None;
                    let mut fallback: Option<(f32, String)> = None;
                    for s in &siblings {
                        if let Some(fname) = s["rfilename"].as_str() {
                            if !fname.to_lowercase().ends_with(".gguf") {
                                continue;
                            }
                            let est_ram = estimate_ram_for_gguf(fname);
                            if est_ram > hw_max_ram {
                                continue;
                            }
                            let q = extract_quant_label(fname);
                            if q.to_uppercase() == hw_quant.to_uppercase() {
                                best_q = Some(q);
                                break;
                            }
                            if fallback.as_ref().map_or(true, |(r, _)| est_ram > *r) {
                                fallback = Some((est_ram, q));
                            }
                        }
                    }
                    if let Some(q) = best_q.or_else(|| fallback.map(|(_, q)| q)) {
                        return format!("hf.co/{}:{}", model_id, q);
                    }
                    return format!("hf.co/{}", model_id);
                }
            }
            format!("hf.co/{}", model_id)
        }
        _ => model_id.to_string(),
    }
}

pub async fn connect_handler(
    State(state): State<AppState>,
    Json(payload): Json<ConnectRequest>,
) -> impl IntoResponse {
    let client = state.client.clone();
    let model_id = payload.model_id.trim().to_string();
    if model_id.is_empty() {
        return Json(ConnectResponse {
            ok: false,
            message: "ID modello vuoto.".to_string(),
            model_id,
        })
        .into_response();
    }
    let hw = state.hw_info.clone();
    let ollama_model_id = resolve_ollama_id(
        &state.config,
        &client,
        hw.max_model_ram_gb,
        &hw.recommended_quant,
        &payload.source,
        &model_id,
    )
    .await;

    let client_clone = client.clone();
    let ollama_url = state.config.ollama.url.clone();
    let model_clone = ollama_model_id.clone();
    tokio::spawn(async move {
        let _ = client_clone
            .post(format!("{}/api/pull", ollama_url))
            .json(&serde_json::json!({ "name": model_clone, "stream": false }))
            .send()
            .await;
    });

    {
        let mut active = state.active_model.lock().await;
        *active = ollama_model_id.clone();
    }

    Json(ConnectResponse {
        ok: true,
        message: format!(
            "Pull avviato per '{}'. SarÃ  disponibile al completamento del download.",
            ollama_model_id
        ),
        model_id: ollama_model_id,
    })
    .into_response()
}

#[derive(Serialize)]
pub struct PullProgress {
    pub status: String,
    pub completed: Option<u64>,
    pub total: Option<u64>,
    pub percent: Option<f64>,
}

pub async fn pull_stream_handler(
    State(state): State<AppState>,
    Json(payload): Json<ConnectRequest>,
) -> impl IntoResponse {
    let client = state.client.clone();
    let model_id = payload.model_id.trim().to_string();

    if model_id.is_empty() {
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(1);
        let _ = tx
            .send(Ok(Event::default().data(
                serde_json::json!({"status": "error", "message": "ID modello vuoto"}).to_string(),
            )))
            .await;
        return Sse::new(ReceiverStream::new(rx)).into_response();
    }

    let hw = state.hw_info.clone();
    let ollama_model_id = resolve_ollama_id(
        &state.config,
        &client,
        hw.max_model_ram_gb,
        &hw.recommended_quant,
        &payload.source,
        &model_id,
    )
    .await;

    let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(32);
    let client_clone = client.clone();
    let ollama_url = state.config.ollama.url.clone();
    let model_clone = ollama_model_id.clone();
    let active_model_ref = state.active_model.clone();

    tokio::spawn(async move {
        {
            let mut active = active_model_ref.lock().await;
            *active = model_clone.clone();
        }

        let res = client_clone
            .post(format!("{}/api/pull", ollama_url))
            .json(&serde_json::json!({ "name": model_clone, "stream": true }))
            .send()
            .await;

        let res = match res {
            Ok(r) => r,
            Err(e) => {
                let _ = tx
                    .send(Ok(Event::default().data(
                        serde_json::json!({"status": "error", "message": e.to_string()})
                            .to_string(),
                    )))
                    .await;
                return;
            }
        };

        let mut stream = res.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let bytes = match chunk {
                Ok(b) => b,
                Err(_) => break,
            };
            buffer.push_str(&String::from_utf8_lossy(&bytes));

            while let Some(pos) = buffer.find('\n') {
                let line: String = buffer.drain(..=pos).collect();
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                    let status = v["status"].as_str().unwrap_or("").to_string();
                    let completed = v["completed"].as_u64();
                    let total = v["total"].as_u64();
                    let percent = match (completed, total) {
                        (Some(c), Some(t)) if t > 0 => Some((c as f64 / t as f64) * 100.0),
                        _ => None,
                    };
                    let progress = PullProgress {
                        status: status.clone(),
                        completed,
                        total,
                        percent,
                    };
                    if let Ok(json) = serde_json::to_string(&progress) {
                        if tx.send(Ok(Event::default().data(json))).await.is_err() {
                            return;
                        }
                    }
                    if status == "success" {
                        return;
                    }
                }
            }
        }
    });

    Sse::new(ReceiverStream::new(rx)).into_response()
}

#[derive(Deserialize)]
pub struct UninstallRequest {
    pub model_id: String,
}

#[derive(Serialize)]
pub struct UninstallResponse {
    pub ok: bool,
    pub message: String,
}

pub async fn uninstall_handler(
    State(state): State<AppState>,
    Json(payload): Json<UninstallRequest>,
) -> impl IntoResponse {
    let model_id = payload.model_id.trim().to_string();
    if model_id.is_empty() {
        return Json(UninstallResponse {
            ok: false,
            message: "ID modello vuoto.".to_string(),
        })
        .into_response();
    }

    let defaults = state.config.default_models();
    if defaults.contains(&model_id) {
        return Json(UninstallResponse {
            ok: false,
            message: format!(
                "'{}' Ã¨ un modello di default e non puÃ² essere disinstallato.",
                model_id
            ),
        })
        .into_response();
    }

    let client = state.client.clone();
    let ollama_url = state.config.ollama.url.clone();
    let res = client
        .delete(format!("{}/api/delete", ollama_url))
        .json(&serde_json::json!({ "model": model_id }))
        .send()
        .await;

    match res {
        Ok(r) if r.status().is_success() => {
            let mut active = state.active_model.lock().await;
            if *active == model_id {
                active.clear();
            }
            Json(UninstallResponse {
                ok: true,
                message: format!("Modello '{}' disinstallato.", model_id),
            })
            .into_response()
        }
        Ok(r) => {
            let status = r.status();
            let body = r.text().await.unwrap_or_default();
            Json(UninstallResponse {
                ok: false,
                message: format!("Ollama ha risposto {}: {}", status, body),
            })
            .into_response()
        }
        Err(e) => Json(UninstallResponse {
            ok: false,
            message: format!("Errore connessione Ollama: {}", e),
        })
        .into_response(),
    }
}

#[derive(Deserialize)]
pub struct ChatRequest {
    pub prompt: Option<String>,
    #[serde(default)]
    pub model_override: Option<String>,
}

pub async fn chat_handler(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Response {
    match chat_handler_inner(state, payload).await {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

async fn chat_handler_inner(
    state: AppState,
    payload: ChatRequest,
) -> Result<Response, AppError> {
    let prompt = match payload.prompt {
        Some(p) if !p.trim().is_empty() => p.trim().to_string(),
        _ => return Err(AppError::BadRequest("Prompt vuoto".to_string())),
    };

    if let Some(craft_response) =
        crate::wordcraft::try_wordcraft_action(&prompt, &state.config).await?
    {
        return Ok(text_response(craft_response));
    }

    if let Some(craft_response) =
        crate::gridcraft::try_gridcraft_action(&prompt, &state.config).await?
    {
        return Ok(text_response(craft_response));
    }

    if prompt.to_lowercase().contains("documento") && prompt.to_lowercase().contains("con:") {
        if let Some(craft_response) =
            crate::wordcraft::create_long_document(&prompt, &state.config).await?
        {
            return Ok(text_response(craft_response));
        }
    }

    if let Some(craft_response) =
        crate::pdfcraft::try_pdfcraft_action(&prompt, &state.config).await?
    {
        return Ok(text_response(craft_response));
    }

    let history = {
        let hist = state.history.lock().await;
        let max_msgs = state.config.history.max_messages * 2;
        let start = if hist.len() > max_msgs {
            let raw = hist.len() - max_msgs;
            if raw < hist.len() && hist[raw].role == "assistant" {
                raw + 1
            } else {
                raw
            }
        } else {
            0
        };
        hist[start..].to_vec()
    };

    let installed = fetch_installed_models(&state.client, &state.config.ollama.url).await;

    let ctx = ChatContext {
        prompt: prompt.clone(),
        history,
        hw_threads: state.hw_info.recommended_threads,
        hw_ctx: state.hw_info.recommended_ctx,
        hw_tier: state.hw_info.tier.clone(),
        model_override: payload.model_override.clone(),
        installed_models: installed,
        config: &state.config,
    };

    let result = run_chat(ctx, &state.client).await?;

    {
        let mut m = state.mode.lock().await;
        *m = result.mode.to_string();
    }

    {
        let mut hist = state.history.lock().await;
        hist.push(ChatMessage {
            role: "user".to_string(),
            content: prompt.clone(),
        });
        hist.push(ChatMessage {
            role: "assistant".to_string(),
            content: format!("[{}]", result.model_used),
        });
    }

    Ok(result.stream)
}

fn text_response(s: String) -> Response {
    Response::builder()
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(Body::from(s))
        .unwrap()
}