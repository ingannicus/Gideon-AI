mod config;
mod error;
mod gridcraft;
mod handlers;
mod hardware;
mod llm;
mod pdfcraft;
mod state;
mod wordcraft;

use axum::{
    routing::{get, post},
    Router,
};
use state::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gideon_ai=info,tower_http=warn".into()),
        )
        .init();

    let cfg = match config::Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[FATAL] {}", e);
            std::process::exit(1);
        }
    };
    if let Err(e) = cfg.ensure_output_dir() {
        eprintln!("[FATAL] impossibile creare output_dir: {}", e);
        std::process::exit(1);
    }

    tracing::info!("Gideon AI v{} — rilevamento hardware...", env!("CARGO_PKG_VERSION"));
    let hw = hardware::HardwareInfo::detect(&cfg.models.nexus, &cfg.models.gemma);
    tracing::info!(
        "CPU: {} ({} core) | RAM libera: {:.1} GB | GPU: {}",
        hw.cpu_brand, hw.cpu_cores, hw.available_ram_gb, hw.gpu_info
    );
    tracing::info!(
        "Tier: {} | Quant: {} | RAM modelli: {:.1} GB",
        hw.tier, hw.recommended_quant, hw.max_model_ram_gb
    );

    // Health-check Ollama
    let probe_client = reqwest::Client::new();
    match probe_client
        .get(format!("{}/api/tags", cfg.ollama.url))
        .send()
        .await
    {
        Ok(r) if r.status().is_success() => tracing::info!("Ollama: OK"),
        _ => tracing::warn!("Ollama: NON raggiungibile su {}", cfg.ollama.url),
    }

    // Health-check CLI MCP
    for (name, path) in [
        ("WordCraft", &cfg.paths.wordcraft_cli),
        ("GridCraft", &cfg.paths.gridcraft_cli),
        ("PdfCraft", &cfg.paths.pdfcraft_cli),
    ] {
        if path.exists() {
            tracing::info!("{} CLI: OK ({})", name, path.display());
        } else {
            tracing::warn!("{} CLI: NON trovato ({})", name, path.display());
        }
    }

    let state = AppState::new(cfg.clone(), hw);

    let app = Router::new()
        .route("/", get(handlers::index_handler))
        .route("/reset", post(handlers::reset_handler))
        .route("/chat", post(handlers::chat_handler))
        .route("/hw", get(handlers::hw_handler))
        .route("/models", get(handlers::models_handler))
        .route("/connect", post(handlers::connect_handler))
        .route("/pull-stream", post(handlers::pull_stream_handler))
        .route("/hf-files", post(handlers::hf_files_handler))
        .route("/uninstall-model", post(handlers::uninstall_handler))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", cfg.server.host, cfg.server.port)
        .parse()
        .expect("addr invalido");

    if cfg.server.open_browser {
        let url = format!("http://localhost:{}", cfg.server.port);
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let _ = webbrowser::open(&url);
        });
    }

    tracing::info!("Server su http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("impossibile installare handler CTRL+C");
    };
    ctrl_c.await;
    tracing::info!("Shutdown richiesto...");
}