use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub ollama: OllamaConfig,
    pub paths: PathsConfig,
    pub models: ModelsConfig,
    pub history: HistoryConfig,
    pub router: RouterConfig,
    pub model_preferences: ModelPreferences,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub open_browser: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OllamaConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PathsConfig {
    pub output_dir: PathBuf,
    pub wordcraft_cli: PathBuf,
    pub gridcraft_cli: PathBuf,
    pub pdfcraft_cli: PathBuf,
    pub pdfcraft_root: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelsConfig {
    pub gemma: String,
    pub nexus: String,
    pub uncensored: String,
    pub fast: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HistoryConfig {
    pub max_messages: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RouterConfig {
    pub model: String,
    pub fallback_to_heuristics: bool,
}

/// Tabella (tier -> modalità -> modello)
#[derive(Debug, Clone, Deserialize)]
pub struct ModelPreferences {
    pub high: HashMap<String, String>,
    pub mid: HashMap<String, String>,
    pub low: HashMap<String, String>,
}

impl ModelPreferences {
    pub fn get(&self, tier: &str, mode: &str) -> Option<&String> {
        match tier {
            "high" => self.high.get(mode),
            "mid" => self.mid.get(mode),
            _ => self.low.get(mode),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let path = std::env::var("GIDEON_CONFIG").unwrap_or_else(|_| "config.toml".to_string());
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Impossibile leggere '{}': {}", path, e))?;
        toml::from_str(&content).map_err(|e| format!("Config '{}' non valida: {}", path, e))
    }

    pub fn default_models(&self) -> Vec<String> {
        vec![
            self.models.gemma.clone(),
            self.models.nexus.clone(),
            self.models.fast.clone(),
            self.models.uncensored.clone(),
        ]
    }

    pub fn ensure_output_dir(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.paths.output_dir)
    }
}