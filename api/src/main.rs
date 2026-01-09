mod auth;
mod config;
mod error;
mod models;
mod routes;
mod services;

use std::net::SocketAddr;

use axum::{middleware, Router};
use eyre::Result;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use auth::{auth_middleware, JwtValidator};
use config::Config;
use services::{EmbedderClient, QdrantService};

#[derive(Clone)]
pub struct AppState {
    pub qdrant: QdrantService,
    pub embedder: EmbedderClient,
    pub jwt_validator: JwtValidator,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ultnote_api=debug,tower_http=debug".into()),
        )
        .init();

    dotenvy::dotenv().ok();

    let config = Config::from_env();

    tracing::info!("Connecting to Qdrant at {}", config.qdrant_url);
    let qdrant = QdrantService::new(&config.qdrant_url).await?;
    qdrant.ensure_collections().await?;
    tracing::info!("Qdrant collections ready");

    let embedder = EmbedderClient::new(config.embedder_url.clone());
    tracing::info!("Embedder client configured for {}", config.embedder_url);

    let jwt_validator = if config.cf_team_domain.is_some() && config.cf_policy_aud.is_some() {
        let validator = JwtValidator::new(
            config.cf_team_domain.clone().unwrap(),
            config.cf_policy_aud.clone().unwrap(),
        );
        tracing::info!("Cloudflare Access JWT validation enabled");
        validator
    } else {
        tracing::warn!("Cloudflare Access JWT validation disabled (missing CF_TEAM_DOMAIN or CF_POLICY_AUD)");
        JwtValidator::disabled()
    };

    let state = AppState {
        qdrant,
        embedder,
        jwt_validator,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(routes::create_router())
        .layer(middleware::from_fn(auth_middleware))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("UltNote API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
