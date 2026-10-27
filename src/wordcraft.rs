use crate::config::Config;
use crate::error::AppError;
use rmcp::{transport::{ConfigureCommandExt, TokioChildProcess}, ServiceExt};
use serde_json::json;
use tokio::process::Command;

pub async fn try_wordcraft_action(prompt: &str, config: &Config) -> Result<Option<String>, AppError> {
    let p = prompt.to_lowercase();
    let is_word_request = p.contains("documento")
        || p.contains("word")
        || p.contains("docx")
        || p.contains("scrivi un file");
    if !is_word_request {
        return Ok(None);
    }

    let cli = config.paths.wordcraft_cli.clone();
    let client = ().serve(
        TokioChildProcess::new(Command::new(&cli).configure(|cmd| {
            cmd.arg("mcp");
        }))
        .map_err(|e| AppError::McpUnavailable(format!("WordCraft: {}", e)))?,
    )
    .await
    .map_err(|e| AppError::McpUnavailable(format!("WordCraft: {}", e)))?;

    let testo = estrai_testo(prompt);
    if testo.is_empty() {
        return Ok(Some(
            "Per favore, specifica cosa scrivere. Es: \"Crea un documento Word con: Ciao mondo\"".to_string(),
        ));
    }

    let nome_file = extract_filename(prompt, "gideon_output.docx");

    let new_doc = rmcp::model::CallToolRequestParam {
        name: "new_document".into(),
        arguments: None,
    };
    client.call_tool(new_doc).await
        .map_err(|e| AppError::Mcp(format!("WordCraft new_document: {}", e)))?;

    let type_params = rmcp::model::CallToolRequestParam {
        name: "type_text".into(),
        arguments: json!({ "text": testo }).as_object().cloned(),
    };
    client.call_tool(type_params).await
        .map_err(|e| AppError::Mcp(format!("WordCraft type_text: {}", e)))?;

    let save_params = rmcp::model::CallToolRequestParam {
        name: "save_document".into(),
        arguments: json!({ "path": nome_file }).as_object().cloned(),
    };
    client.call_tool(save_params).await
        .map_err(|e| AppError::Mcp(format!("WordCraft save_document: {}", e)))?;

    Ok(Some(format!(
        "✅ Documento creato e salvato come {}\n\nTesto scritto:\n{}",
        nome_file, testo
    )))
}

fn estrai_testo(prompt: &str) -> String {
    for sep in ["con:", "che dice:", ":", "su "] {
        if let Some(pos) = prompt.find(sep) {
            let testo = prompt[pos + sep.len()..].trim();
            if !testo.is_empty() {
                return testo.to_string();
            }
        }
    }
    prompt.trim().to_string()
}

fn extract_filename(prompt: &str, default: &str) -> String {
    for word in prompt.split_whitespace() {
        let lower = word.to_lowercase();
        if lower.ends_with(".docx") || lower.ends_with(".doc") {
            return word
                .trim_matches(|c| c == '"' || c == '\'' || c == '`')
                .to_string();
        }
    }
    default.to_string()
}

pub async fn create_long_document(prompt: &str, config: &Config) -> Result<Option<String>, AppError> {
    let testo = if let Some(pos) = prompt.find("con:") {
        prompt[pos + 4..].trim().to_string()
    } else {
        prompt.to_string()
    };

    if testo.len() < 100 {
        return Ok(None);
    }

    let nome = extract_filename(prompt, "gideon_output.docx");
    let nome_txt = nome.replace(".docx", ".txt").replace(".doc", ".txt");

    let out_dir = &config.paths.output_dir;
    std::fs::create_dir_all(out_dir)?;
    let txt_path = out_dir.join(&nome_txt);
    let docx_path = out_dir.join(&nome);

    std::fs::write(&txt_path, &testo)?;

    let cli = config.paths.wordcraft_cli.clone();
    let client = ().serve(
        TokioChildProcess::new(Command::new(&cli).configure(|cmd| {
            cmd.arg("mcp");
        }))
        .map_err(|e| AppError::McpUnavailable(format!("WordCraft: {}", e)))?,
    )
    .await
    .map_err(|e| AppError::McpUnavailable(format!("WordCraft: {}", e)))?;

    let open_params = rmcp::model::CallToolRequestParam {
        name: "open_document".into(),
        arguments: json!({ "path": txt_path.to_string_lossy() }).as_object().cloned(),
    };
    client.call_tool(open_params).await
        .map_err(|e| AppError::Mcp(format!("WordCraft open_document: {}", e)))?;

    let save_params = rmcp::model::CallToolRequestParam {
        name: "save_document".into(),
        arguments: json!({ "path": docx_path.to_string_lossy() }).as_object().cloned(),
    };
    client.call_tool(save_params).await
        .map_err(|e| AppError::Mcp(format!("WordCraft save_document: {}", e)))?;

    Ok(Some(format!(
        "✅ Documento lungo creato: {} ({} caratteri)",
        docx_path.display(),
        testo.len()
    )))
}