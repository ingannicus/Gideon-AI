// src/pdfcraft.rs
use rmcp::{ServiceExt, transport::{TokioChildProcess, ConfigureCommandExt}};
use tokio::process::Command;

pub async fn try_pdfcraft_action(prompt: &str) -> Option<String> {
    let p = prompt.to_lowercase();
    let is_pdf_request = p.contains("pdf") || p.contains("estrai") || p.contains("analizza");

    if !is_pdf_request {
        return None;
    }

    let filename = extract_pdf_filename(prompt).unwrap_or_else(|| "documento.pdf".to_string());

    // Avvia PdfCraft con --root puntato alla cartella del progetto
    let client = match ().serve(
        TokioChildProcess::new(
            Command::new(r"C:\Program Files\PdfCraft\pdfcraft-cli.exe").configure(|cmd| {
                cmd.arg("mcp");
                cmd.arg("--root").arg(r"C:\Users\degia\Desktop\GideonTest");
            })
        ).ok()?
    ).await {
        Ok(c) => c,
        Err(_) => return Some("⚠️ PdfCraft non avviato.".to_string()),
    };

    // 1. Apri il documento
    let open_params = rmcp::model::CallToolRequestParam {
        name: "doc_open".into(),
        arguments: serde_json::json!({ "path": filename }).as_object().cloned(),
    };

    let open_result = match client.call_tool(open_params).await {
        Ok(res) => res,
        Err(e) => return Some(format!("❌ Errore apertura PDF '{}': {}", filename, e)),
    };

    // 2. Estrai l'ID del documento
    let doc_id = match extract_doc_id(&open_result) {
        Some(id) => id,
        None => {
            return Some(format!(
                "❌ PdfCraft ha aperto il PDF ma non è stato possibile ottenere l'ID.\nRisposta: {:?}",
                open_result
            ));
        }
    };

    // 3. Ottieni il numero di pagine (con fallback a 1)
    let info_params = rmcp::model::CallToolRequestParam {
        name: "doc_info".into(),
        arguments: serde_json::json!({ "doc": doc_id }).as_object().cloned(),
    };
    let num_pages = match client.call_tool(info_params).await {
        Ok(res) => extract_page_count(&res).unwrap_or(1),
        Err(_) => 1,
    };

    // 4. Estrai il testo da tutte le pagine
    let pages: Vec<i64> = (1..=num_pages).collect();
    let extract_params = rmcp::model::CallToolRequestParam {
        name: "text_extract".into(),
        arguments: serde_json::json!({
            "doc": doc_id,
            "pages": pages
        }).as_object().cloned(),
    };

    match client.call_tool(extract_params).await {
        Ok(res) => {
            // Estrai il testo pulito, pagina per pagina
            let mut testo_pulito = String::new();
            for content in &res.content {
                if let rmcp::model::RawContent::Text(text_content) = &content.raw {
                    // Il testo è un JSON con struttura {"pages": [{"page": 1, "text": "..."}]}
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text_content.text) {
                        if let Some(pages) = json.get("pages").and_then(|p| p.as_array()) {
                            for page in pages {
                                if let Some(text) = page.get("text").and_then(|t| t.as_str()) {
                                    testo_pulito.push_str(text);
                                    testo_pulito.push_str("\n\n");
                                }
                            }
                        }
                    } else {
                        // Se non è JSON, usa il testo così com'è
                        testo_pulito.push_str(&text_content.text);
                    }
                }
            }
            Some(format!(
                "✅ Testo estratto da '{}':\n\n{}",
                filename,
                testo_pulito.trim()
            ))
        }
        Err(e) => Some(format!(
            "❌ Errore estrazione da '{}': {}\n\nAssicurati che il file esista e che PdfCraft sia aggiornato.",
            filename, e
        )),
    }
}

/// Estrae l'ID del documento dalla risposta di `doc_open`.
fn extract_doc_id(result: &rmcp::model::CallToolResult) -> Option<i64> {
    for content in &result.content {
        if let rmcp::model::RawContent::Text(text_content) = &content.raw {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text_content.text) {
                if let Some(id) = json.get("doc").and_then(|v| v.as_i64()) {
                    return Some(id);
                }
                if let Some(id) = json.get("doc_id").and_then(|v| v.as_i64()) {
                    return Some(id);
                }
                if let Some(id) = json.get("result").and_then(|r| r.get("doc")).and_then(|v| v.as_i64()) {
                    return Some(id);
                }
            }
        }
    }
    None
}

/// Estrae il numero di pagine dalla risposta di `doc_info`.
fn extract_page_count(result: &rmcp::model::CallToolResult) -> Option<i64> {
    for content in &result.content {
        if let rmcp::model::RawContent::Text(text_content) = &content.raw {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text_content.text) {
                if let Some(count) = json.get("pages").and_then(|v| v.as_i64()) {
                    return Some(count);
                }
                if let Some(count) = json.get("page_count").and_then(|v| v.as_i64()) {
                    return Some(count);
                }
                if let Some(count) = json.get("result").and_then(|r| r.get("pages")).and_then(|v| v.as_i64()) {
                    return Some(count);
                }
            }
        }
    }
    None
}

/// Estrae il nome di un file .pdf dal prompt.
/// Se il file esiste nella cartella corrente, restituisce il percorso assoluto.
fn extract_pdf_filename(prompt: &str) -> Option<String> {
    for word in prompt.split_whitespace() {
        if word.to_lowercase().ends_with(".pdf") {
            let name = word.trim_matches(|c| c == '"' || c == '\'' || c == '`').to_string();
            if std::path::Path::new(&name).exists() {
                if let Ok(cwd) = std::env::current_dir() {
                    let full = cwd.join(&name);
                    return Some(full.to_string_lossy().to_string());
                }
            }
            return Some(name);
        }
    }
    None
}