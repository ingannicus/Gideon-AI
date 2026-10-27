use crate::config::Config;
use crate::error::AppError;
use rmcp::{transport::{ConfigureCommandExt, TokioChildProcess}, ServiceExt};
use tokio::process::Command;

pub async fn try_pdfcraft_action(prompt: &str, config: &Config) -> Result<Option<String>, AppError> {
    let p = prompt.to_lowercase();
    let is_pdf_request = p.contains("pdf") || p.contains("estrai") || p.contains("analizza");
    if !is_pdf_request {
        return Ok(None);
    }

    let filename = extract_pdf_filename(prompt).unwrap_or_else(|| "documento.pdf".to_string());
    let cli = config.paths.pdfcraft_cli.clone();
    let root = config.paths.pdfcraft_root.clone();

    let client = ().serve(
        TokioChildProcess::new(Command::new(&cli).configure(|cmd| {
            cmd.arg("mcp");
            cmd.arg("--root").arg(&root);
        }))
        .map_err(|e| AppError::McpUnavailable(format!("PdfCraft: {}", e)))?,
    )
    .await
    .map_err(|e| AppError::McpUnavailable(format!("PdfCraft: {}", e)))?;

    let open_params = rmcp::model::CallToolRequestParam {
        name: "doc_open".into(),
        arguments: serde_json::json!({ "path": filename }).as_object().cloned(),
    };
    let open_result = client
        .call_tool(open_params)
        .await
        .map_err(|e| AppError::Mcp(format!("PdfCraft doc_open '{}': {}", filename, e)))?;

    let doc_id = extract_doc_id(&open_result).ok_or_else(|| {
        AppError::Mcp(format!(
            "PdfCraft ha aperto il PDF ma non è stato possibile ottenere l'ID. Risposta: {:?}",
            open_result
        ))
    })?;

    let info_params = rmcp::model::CallToolRequestParam {
        name: "doc_info".into(),
        arguments: serde_json::json!({ "doc": doc_id }).as_object().cloned(),
    };
    let num_pages = match client.call_tool(info_params).await {
        Ok(res) => extract_page_count(&res).unwrap_or(1),
        Err(_) => 1,
    };

    let pages: Vec<i64> = (1..=num_pages).collect();
    let extract_params = rmcp::model::CallToolRequestParam {
        name: "text_extract".into(),
        arguments: serde_json::json!({
            "doc": doc_id,
            "pages": pages
        })
        .as_object()
        .cloned(),
    };

    let res = client
        .call_tool(extract_params)
        .await
        .map_err(|e| AppError::Mcp(format!("PdfCraft text_extract: {}", e)))?;

    let mut testo_pulito = String::new();
    for content in &res.content {
        if let rmcp::model::RawContent::Text(text_content) = &content.raw {
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
                testo_pulito.push_str(&text_content.text);
            }
        }
    }

    Ok(Some(format!(
        "✅ Testo estratto da '{}':\n\n{}",
        filename,
        testo_pulito.trim()
    )))
}

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
                if let Some(id) = json
                    .get("result")
                    .and_then(|r| r.get("doc"))
                    .and_then(|v| v.as_i64())
                {
                    return Some(id);
                }
            }
        }
    }
    None
}

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
                if let Some(count) = json
                    .get("result")
                    .and_then(|r| r.get("pages"))
                    .and_then(|v| v.as_i64())
                {
                    return Some(count);
                }
            }
        }
    }
    None
}

fn extract_pdf_filename(prompt: &str) -> Option<String> {
    for word in prompt.split_whitespace() {
        if word.to_lowercase().ends_with(".pdf") {
            let name = word
                .trim_matches(|c| c == '"' || c == '\'' || c == '`')
                .to_string();
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