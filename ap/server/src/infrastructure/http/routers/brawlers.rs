use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router, State,
};
use serde_json::json;

use crate::{
    application::use_cases::authentication::AuthenticationUseCase,
    domain::{
        repositories::brawlers::BrawlerRepository,
        value_objects::brawler_model::{LoginModel, RegisterBrawlerModel},
    },
    infrastructure::database::brawlers::PostgresBrawlerRepository,
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn router(db_pool: Arc<PgPool>) -> Router {
    let brawler_repository = Arc::new(PostgresBrawlerRepository::new(db_pool.clone()));
    let authentication_use_case = Arc::new(AuthenticationUseCase::new(brawler_repository));

    Router::new()
        .route(
            "/login",
            post(login::<AuthenticationUseCase<PostgresBrawlerRepository>>),
        )
        .with_state(Arc::clone(&authentication_use_case))
        .route(
            "/register",
            post(register::<AuthenticationUseCase<PostgresBrawlerRepository>>),
        )
        .with_state(Arc::clone(&authentication_use_case))
}

pub async fn login<T>(
    State(auth_use_case): State<Arc<T>>,
    Json(payload): Json<LoginModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match auth_use_case
        .login(payload)
        .await
    {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn register<T>(
    State(auth_use_case): State<Arc<T>>,
    Json(payload): Json<RegisterBrawlerModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match auth_use_case
        .register(payload)
        .await
    {
        Ok(user_id) => (
            StatusCode::CREATED,
            Json(json!({ "user_id": user_id })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn upload_avatar<T>(
    State(brawler_repository): State<Arc<T>>,
    Path(user_id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    let base64_image = match payload.get("base64_image").and_then(|v| v.as_str()) {
        Some(img) => img.to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "base64_image is required" })),
            )
                .into_response()
        }
    };

    match brawler_repository
        .upload_base64img(user_id, base64_image, None)
        .await
    {
        Ok(uploaded_img) => (StatusCode::OK, Json(uploaded_img)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
