use crate::config::Config;
use crate::hardware::HardwareInfo;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub history: Arc<Mutex<Vec<ChatMessage>>>,
    pub mode: Arc<Mutex<String>>,
    pub client: Client,
    pub hw_info: Arc<HardwareInfo>,
    pub active_model: Arc<Mutex<String>>,
    pub config: Arc<Config>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl AppState {
    pub fn new(config: Config, hw_info: HardwareInfo) -> Self {
        Self {
            history: Arc::new(Mutex::new(Vec::new())),
            mode: Arc::new(Mutex::new("general".to_string())),
            client: Client::new(),
            hw_info: Arc::new(hw_info),
            active_model: Arc::new(Mutex::new(String::new())),
            config: Arc::new(config),
        }
    }
}