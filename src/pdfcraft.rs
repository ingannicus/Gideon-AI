// src/pdfcraft.rs
use rmcp::{ServiceExt, transport::{TokioChildProcess, ConfigureCommandExt}};
use tokio::process::Command;

pub async fn try_pdfcraft_action(prompt: &str) -> Option<String> {
    let p = prompt.to_lowercase();
    let is_pdf_request = p.contains("pdf") || p.contains("estrai") || p.contains("unisci") || p.contains("dividi");

    if !is_pdf_request {
        return None;
    }

    let filename = extract_pdf_filename(prompt).unwrap_or_else(|| "documento.pdf".to_string());

    let client = match ().serve(
        TokioChildProcess::new(
            Command::new(r"C:\Program Files\PdfCraft\pdfcraft-cli.exe").configure(|cmd| {
                cmd.arg("mcp");
            })
        ).ok()?
    ).await {
        Ok(c) => c,
        Err(_) => return Some("âš ï¸ PdfCraft non avviato.".to_string()),
    };

    let extract_params = rmcp::model::CallToolRequestParam {
        name: "text_extract".into(),
        arguments: serde_json::json!({ "path": filename }).as_object().cloned(),
    };

    match client.call_tool(extract_params).await {
        Ok(res) => Some(format!("âœ… Testo estratto da '{}':\n\n{:?}", filename, res)),
        Err(e) => Some(format!(
            "âŒ Errore estrazione da '{}': {}\n\nAssicurati che il file esista nella cartella del progetto.",
            filename, e
        )),
    }
}

fn extract_pdf_filename(prompt: &str) -> Option<String> {
    for word in prompt.split_whitespace() {
        if word.to_lowercase().ends_with(".pdf") {
            return Some(word.trim_matches(|c| c == '"' || c == '\'' || c == '`').to_string());
        }
    }
    None
}