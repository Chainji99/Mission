use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{post, get},
    Router,
};
use serde::Deserialize;

use crate::{
    application::use_cases::authentication::{AuthenticationUseCase, LoginModel},
    infrastructure::{
        database::{
            repositories::brawlers::BrawlerPostgres,
            postgresql_connection::PgPoolSquad,
        },
        services::{
            email_service::EmailService,
            google_auth_service::GoogleAuthService,
        },
    },
};

pub fn router(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawler_repository = BrawlerPostgres::new(db_pool);
    let email_service = Arc::new(EmailService::new());
    // In production, handle error properly. For now, panic if env vars missing is okay or default to mock.
    let google_auth_service = Arc::new(GoogleAuthService::new().unwrap_or_else(|e| {
        println!("Warning: Google Auth Service init failed: {}", e);
        // Fallback to a safe dummy if initialization fails (e.g. malformed URL)
        // This prevents the server from crashing loop.
        match GoogleAuthService::new() {
             Ok(s) => s,
             Err(_) => panic!("Critical: Google Auth Service failed to initialize even with defaults. Check URL format."),
        }
    }));

    let auth_use_case = Arc::new(AuthenticationUseCase::new(
        Arc::new(brawler_repository),
        email_service,
        google_auth_service,
    ));

    Router::new()
        .route("/login", post(login))
        .route("/google/url", get(google_url))
        .route("/google/callback", post(google_callback))
        .route("/request-reset", post(request_reset))
        .route("/reset", post(reset_password))
        .with_state(auth_use_case)
}

pub async fn login(
    State(use_case): State<Arc<AuthenticationUseCase<BrawlerPostgres>>>,
    Json(payload): Json<LoginModel>,
) -> impl IntoResponse {
    match use_case.login(payload).await {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn google_url(
    State(use_case): State<Arc<AuthenticationUseCase<BrawlerPostgres>>>,
) -> impl IntoResponse {
    let (url, _) = use_case.get_google_auth_url();
    (StatusCode::OK, Json(serde_json::json!({ "url": url }))).into_response()
}

#[derive(Deserialize)]
pub struct GoogleCallbackModel {
    code: String,
}

pub async fn google_callback(
    State(use_case): State<Arc<AuthenticationUseCase<BrawlerPostgres>>>,
    Json(payload): Json<GoogleCallbackModel>,
) -> impl IntoResponse {
    match use_case.login_with_google(payload.code).await {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
pub struct RequestResetModel {
    pub username: String,
}

pub async fn request_reset(
    State(use_case): State<Arc<AuthenticationUseCase<BrawlerPostgres>>>,
    Json(payload): Json<RequestResetModel>,
) -> impl IntoResponse {
    match use_case.request_password_reset(payload.username).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
pub struct ResetPasswordModel {
    pub token: String,
    pub new_password: String,
}

pub async fn reset_password(
    State(use_case): State<Arc<AuthenticationUseCase<BrawlerPostgres>>>,
    Json(payload): Json<ResetPasswordModel>,
) -> impl IntoResponse {
    match use_case.reset_password(payload.token, payload.new_password).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
