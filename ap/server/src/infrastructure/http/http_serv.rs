use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{
    routing::get,
    Router,
    http::{
        Method, StatusCode,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    config::config_model::DotEnvyConfig,
    infrastructure::database::postgresql_connection::PgPoolSquad,
};

use super::routers;

fn static_serve() -> Router {
    let dir = "statics";

    let service = ServeDir::new(dir).not_found_service(ServeFile::new(format!("{dir}/index.html")));

    Router::new().fallback_service(service)
}

fn api_serve(db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new()
        .nest("/authentication", routers::brawlers::router(db_pool.clone()))
        .fallback(|| async { (StatusCode::NOT_FOUND, "API not found") })
}

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new()
        .merge(static_serve())
        .nest("/api/v1", api_serve(db_pool.clone()))
        .route("/health", get(|| async { "OK" }))
        .layer(tower_http::limit::RequestRateLimitLayer::new(
            config.server.rate_limit.requests,
            Duration::from_secs(config.server.rate_limit.per_seconds),
        ))
        .layer(RequestBodyLimitLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
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
                .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
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
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("Fail ctrl + c") };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Receive ctrl + c signal"),
        _ = terminate => info!("Receive terminate signal"),
    }
}