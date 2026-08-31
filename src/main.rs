#[allow(dead_code)]
mod auth;
mod config;
#[allow(dead_code)]
mod handlers;
#[allow(dead_code)]
mod metrics;
#[allow(dead_code)]
mod storage;
#[allow(dead_code)]
mod templates;

use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use config::{Cli, Config};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Combined application state
#[derive(Clone)]
pub struct AppState {
    pub db: storage::SharedDb,
    pub auth: auth::AuthState,
    pub metrics: metrics::MetricsState,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "openmetrics=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load config
    let cli = Cli::parse();
    let config = Config::load(&cli.config)?;

    tracing::info!("Starting OpenMetrics on {}", config.listen_addr());
    tracing::info!("Polling interval: {}s", config.metrics.polling_interval_secs);
    tracing::info!("Database: {}", config.database.db_path);

    // Initialize storage
    let db = storage::initialize(&config.database.db_path)?;

    // Initialize auth state
    let auth_state = auth::AuthState::from_config(&config.auth);

    // Initialize metrics collection
    let metrics_state = metrics::new_metrics_state();
    metrics::spawn_collector(metrics_state.clone(), config.metrics.polling_interval_secs);

    // Build combined state
    let state = AppState {
        db: db.clone(),
        auth: auth_state,
        metrics: metrics_state,
    };

    // Build application routes
    let app = Router::new()
        // Auth routes
        .route("/", get(handlers::dashboard::dashboard_page))
        .route("/login", get(auth::handlers::login_page))
        .route("/login", post(auth::handlers::login_submit))
        .route("/logout", get(auth::handlers::logout))

        // Dashboard partials (HTMX endpoints)
        .route("/partials/header", get(handlers::dashboard::header_panel))
        .route("/partials/cpu", get(handlers::dashboard::cpu_panel))
        .route("/partials/memory", get(handlers::dashboard::memory_panel))
        .route("/partials/disk", get(handlers::dashboard::disk_panel))
        .route("/partials/network", get(handlers::dashboard::network_panel))
        .route("/partials/processes", get(handlers::dashboard::process_table))

        // API routes
        .route("/api/v1/metrics/current", get(handlers::api::current_metrics))
        .route("/api/v1/metrics/history", get(handlers::api::history_metrics))
        .route("/api/v1/metrics/{metric_type}", get(handlers::api::specific_metric))
        .route("/api/v1/docs", get(handlers::api::api_docs))

        // Static files
        .fallback_service(ServeDir::new("static"))

        // Shared state
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(config.listen_addr()).await?;
    tracing::info!("Server listening on {}", config.listen_addr());

    axum::serve(listener, app).await?;

    Ok(())
}
