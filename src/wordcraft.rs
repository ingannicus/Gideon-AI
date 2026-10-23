// src/wordcraft.rs
use rmcp::{ServiceExt, transport::{TokioChildProcess, ConfigureCommandExt}};
use tokio::process::Command;
use serde_json::json;

/// Prova a gestire una richiesta usando WordCraft.
/// Restituisce `Some(risposta)` se WordCraft ha gestito la richiesta,
/// `None` se Gideon deve procedere normalmente con Ollama.
pub async fn try_wordcraft_action(prompt: &str) -> Option<String> {
    let p = prompt.to_lowercase();
    let is_word_request = p.contains("documento")
        || p.contains("word")
        || p.contains("docx")
        || p.contains("scrivi un file");

    if !is_word_request {
        return None;
    }

    // Avvia il server MCP di WordCraft
    let client = match ().serve(
        TokioChildProcess::new(
            Command::new(r"C:\Program Files\WordCraft\wordcraft-cli.exe").configure(|cmd| {
                cmd.arg("mcp");
                // Scommenta la riga sotto se il salvataggio dà errore "path non consentito"
                // cmd.arg("--automation-write-root").arg(r"C:\Users\degia\Desktop\GideonTest");
            })
        ).ok()?
    ).await {
        Ok(c) => c,
        Err(_) => return Some("⚠️ WordCraft non avviato o non trovato.".to_string()),
    };

    // Estrae il testo da scrivere dal prompt
    let testo = estrai_testo(prompt);
    if testo.is_empty() {
        return Some(
            "Per favore, specifica cosa scrivere. Es: \"Crea un documento Word con: Ciao mondo\""
                .to_string(),
        );
    }

    // Estrae il nome file dal prompt (es. "report.docx"), altrimenti usa il default
    let nome_file = extract_filename(prompt, "gideon_output.docx");

    // 1) Crea un nuovo documento (senza argomenti)
    let new_doc = rmcp::model::CallToolRequestParam {
        name: "new_document".into(),
        arguments: None,
    };
    if let Err(e) = client.call_tool(new_doc).await {
        return Some(format!("❌ Errore creazione documento: {}", e));
    }

    // 2) Inserisce il testo (con argomento)
    let type_params = rmcp::model::CallToolRequestParam {
        name: "type_text".into(),
        arguments: json!({ "text": testo }).as_object().cloned(),
    };
    if let Err(e) = client.call_tool(type_params).await {
        return Some(format!("❌ Errore scrittura testo: {}", e));
    }

    // 3) Salva il file (con argomento, usando il nome estratto dal prompt)
    let save_params = rmcp::model::CallToolRequestParam {
        name: "save_document".into(),
        arguments: json!({ "path": nome_file }).as_object().cloned(),
    };
    match client.call_tool(save_params).await {
        Ok(_) => Some(format!(
            "✅ Documento creato e salvato come {}\n\nTesto scritto:\n{}",
            nome_file, testo
        )),
        Err(e) => Some(format!("❌ Errore salvataggio: {}", e)),
    }
}

/// Estrae il testo da scrivere dal prompt dell'utente.
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

/// Estrae il nome di un file .docx/.doc dal prompt.
/// Se non lo trova, restituisce `default`.
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

/// Scrive un testo lungo in un file .txt, poi lo fa aprire a WordCraft
/// e lo salva come .docx. Utile per testi > 100 caratteri.
pub async fn create_long_document(prompt: &str) -> Option<String> {
    let testo = if let Some(pos) = prompt.find("con:") {
        prompt[pos + 4..].trim().to_string()
    } else {
        prompt.to_string()
    };

    if testo.len() < 100 {
        return None;
    }

    let nome = extract_filename(prompt, "gideon_output.docx");
    let nome_txt = nome.replace(".docx", ".txt").replace(".doc", ".txt");

    if let Err(e) = std::fs::write(&nome_txt, &testo) {
        return Some(format!("❌ Errore scrittura file: {}", e));
    }

    let client = match ().serve(
        TokioChildProcess::new(
            Command::new(r"C:\Program Files\WordCraft\wordcraft-cli.exe").configure(|cmd| {
                cmd.arg("mcp");
            })
        ).ok()?
    ).await {
        Ok(c) => c,
        Err(_) => return Some(format!("⚠️ File .txt creato ma WordCraft non avviato: {}", nome_txt)),
    };

    let open_params = rmcp::model::CallToolRequestParam {
        name: "open_document".into(),
        arguments: json!({ "path": nome_txt }).as_object().cloned(),
    };
    if let Err(e) = client.call_tool(open_params).await {
        return Some(format!("❌ File .txt creato ma errore apertura: {}", e));
    }

    let save_params = rmcp::model::CallToolRequestParam {
        name: "save_document".into(),
        arguments: json!({ "path": nome }).as_object().cloned(),
    };
    match client.call_tool(save_params).await {
        Ok(_) => Some(format!(
            "✅ Documento lungo creato: {} ({} caratteri)",
            nome, testo.len()
        )),
        Err(e) => Some(format!("❌ File .txt creato ma errore salvataggio .docx: {}", e)),
    }
}
