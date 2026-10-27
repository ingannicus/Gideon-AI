use crate::config::Config;
use crate::error::AppError;
use rmcp::{transport::{ConfigureCommandExt, TokioChildProcess}, ServiceExt};
use tokio::process::Command;

pub async fn try_gridcraft_action(prompt: &str, config: &Config) -> Result<Option<String>, AppError> {
    let p = prompt.to_lowercase();
    let is_grid_request = p.contains("foglio")
        || p.contains("excel")
        || p.contains("tabella")
        || p.contains("cella");
    if !is_grid_request {
        return Ok(None);
    }

    let cli = config.paths.gridcraft_cli.clone();
    let client = ().serve(
        TokioChildProcess::new(Command::new(&cli).configure(|cmd| {
            cmd.arg("mcp");
        }))
        .map_err(|e| AppError::McpUnavailable(format!("GridCraft: {}", e)))?,
    )
    .await
    .map_err(|e| AppError::McpUnavailable(format!("GridCraft: {}", e)))?;

    let data = extract_list_from_prompt(prompt);
    if data.is_empty() {
        return Ok(Some(
            "Specifica i dati da scrivere. Es: \"Crea un foglio con Mario, Luigi, Anna\"".to_string(),
        ));
    }

    let end_row = data.len();
    let values: Vec<Vec<serde_json::Value>> = data
        .iter()
        .map(|v| vec![serde_json::Value::String(v.clone())])
        .collect();

    let write_params = rmcp::model::CallToolRequestParam {
        name: "write_range".into(),
        arguments: serde_json::json!({
            "range": format!("A1:A{}", end_row),
            "values": values
        })
        .as_object()
        .cloned(),
    };
    client
        .call_tool(write_params)
        .await
        .map_err(|e| AppError::Mcp(format!("GridCraft write_range: {}", e)))?;

    let save_params = rmcp::model::CallToolRequestParam {
        name: "save_workbook".into(),
        arguments: serde_json::json!({ "path": "gideon_grid.xlsx" })
            .as_object()
            .cloned(),
    };
    client
        .call_tool(save_params)
        .await
        .map_err(|e| AppError::Mcp(format!("GridCraft save_workbook: {}", e)))?;

    Ok(Some(format!(
        "✅ Foglio salvato come gideon_grid.xlsx con {} righe: {}",
        end_row,
        data.join(", ")
    )))
}

fn extract_list_from_prompt(prompt: &str) -> Vec<String> {
    for sep in ["con:", "con ", ":"] {
        if let Some(pos) = prompt.to_lowercase().find(sep) {
            let parte = &prompt[pos + sep.len()..];
            let items: Vec<String> = parte
                .split(&[',', ';'][..])
                .map(|s| {
                    s.trim()
                        .trim_matches(|c| c == '"' || c == '\'' || c == '.')
                        .to_string()
                })
                .filter(|s| !s.is_empty())
                .collect();
            if !items.is_empty() {
                return items;
            }
        }
    }
    Vec::new()
}