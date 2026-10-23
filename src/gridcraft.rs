// src/gridcraft.rs
use rmcp::{ServiceExt, transport::{TokioChildProcess, ConfigureCommandExt}};
use tokio::process::Command;

pub async fn try_gridcraft_action(prompt: &str) -> Option<String> {
    let p = prompt.to_lowercase();
    let is_grid_request = p.contains("foglio") || p.contains("excel") || p.contains("tabella") || p.contains("cella");

    if !is_grid_request {
        return None;
    }

    let client = match ().serve(
        TokioChildProcess::new(
            Command::new(r"C:\Program Files\GridCraft\gridcraft-cli.exe").configure(|cmd| {
                cmd.arg("mcp");
            })
        ).ok()?
    ).await {
        Ok(c) => c,
        Err(_) => return Some("âš ï¸ GridCraft non avviato.".to_string()),
    };

    let data = extract_list_from_prompt(prompt);
    if data.is_empty() {
        return Some("Specifica i dati da scrivere. Es: \"Crea un foglio con Mario, Luigi, Anna\"".to_string());
    }

    let end_row = data.len();
    let values: Vec<Vec<serde_json::Value>> = data.iter()
        .map(|v| vec![serde_json::Value::String(v.clone())])
        .collect();

    let write_params = rmcp::model::CallToolRequestParam {
        name: "write_range".into(),
        arguments: serde_json::json!({
            "range": format!("A1:A{}", end_row),
            "values": values
        }).as_object().cloned(),
    };

    if let Err(e) = client.call_tool(write_params).await {
        return Some(format!("âŒ Errore scrittura: {}", e));
    }

    let save_params = rmcp::model::CallToolRequestParam {
        name: "save_workbook".into(),
        arguments: serde_json::json!({ "path": "gideon_grid.xlsx" }).as_object().cloned(),
    };

    match client.call_tool(save_params).await {
        Ok(_) => Some(format!("âœ… Foglio salvato come gideon_grid.xlsx con {} righe.", end_row)),
        Err(e) => Some(format!("âŒ Errore salvataggio: {}", e)),
    }
}

fn extract_list_from_prompt(prompt: &str) -> Vec<String> {
    for sep in ["con:", "con ", ":"] {
        if let Some(pos) = prompt.to_lowercase().find(sep) {
            let parte = &prompt[pos + sep.len()..];
            let items: Vec<String> = parte
                .split(&[',', ';'][..])
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\'' || c == '.').to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !items.is_empty() {
                return items;
            }
        }
    }
    Vec::new()
}