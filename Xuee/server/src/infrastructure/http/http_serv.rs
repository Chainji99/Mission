use std::{net::SocketAddr, sync::Arc};
use anyhow::Result;
use axum::{
    Router,
    http::{Method, StatusCode},
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    config::config_model::DotEnvyConfig,
    infrastructure::{
        database::postgresql_connection::PgPoolSquad,
        http::routers,
    },
};

fn static_serve() -> Router {
    let dir = "statics";
    let service = ServeDir::new(dir).not_found_service(ServeFile::new(format!("{dir}/index.html")));
    Router::new().fallback_service(service)
}

fn api_serve(db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new()
        .nest("/v1", Router::new()
            .nest("/authentication", routers::authentication::router(db_pool.clone()))
            .nest("/brawlers", routers::brawlers::router(db_pool.clone()))
            .nest("/missions", routers::missions::router(db_pool.clone()))
            .nest("/mission-management", routers::mission_management::router(db_pool.clone()))
            .nest("/debug", routers::debug::router(db_pool.clone()))
            .nest("/cards", routers::cards::router(db_pool.clone()))

        )
        .fallback(|| async { (StatusCode::NOT_FOUND, "API route not found") })
}

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new()
        .merge(static_serve())
        .nest("/api", api_serve(db_pool))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_origin(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = TcpListener::bind(addr).await?;

    info!("Server start on port {}", config.server.port);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
