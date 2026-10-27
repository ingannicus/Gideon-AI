use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[allow(dead_code)]
    #[error("Configurazione non valida: {0}")]
    Config(String),

    #[error("Ollama non raggiungibile: {0}")]
    OllamaUnavailable(String),

    #[error("Ollama ha risposto con errore: {0}")]
    OllamaError(String),

    #[error("CLI MCP non disponibile: {0}")]
    McpUnavailable(String),

    #[error("Errore MCP: {0}")]
    Mcp(String),

    #[error("Richiesta non valida: {0}")]
    BadRequest(String),

    #[error("Errore I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("Errore interno: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            AppError::Config(m) => (StatusCode::INTERNAL_SERVER_ERROR, m.clone()),
            AppError::OllamaUnavailable(m) => (StatusCode::SERVICE_UNAVAILABLE, m.clone()),
            AppError::OllamaError(m) => (StatusCode::BAD_GATEWAY, m.clone()),
            AppError::McpUnavailable(m) => (StatusCode::SERVICE_UNAVAILABLE, m.clone()),
            AppError::Mcp(m) => (StatusCode::INTERNAL_SERVER_ERROR, m.clone()),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m.clone()),
            AppError::Io(m) => (StatusCode::INTERNAL_SERVER_ERROR, m.to_string()),
            AppError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m.clone()),
        };
        tracing::error!(target: "gideon", "{}", self);
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

#[allow(dead_code)]
pub type AppResult<T> = Result<T, AppError>;