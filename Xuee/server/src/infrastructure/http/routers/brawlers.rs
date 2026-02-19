use std::sync::Arc;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::post,
    Router,
    middleware,
};

use crate::{
    application::use_cases::brawlers::BrawlersUseCase,
    domain::value_objects::brawler_model::{RegisterBrawlerModel, AvatarUploadRequest, UpdateDisplayNameRequest},
    infrastructure::{
        database::{
            repositories::brawlers::BrawlerPostgres,
            postgresql_connection::PgPoolSquad,
        },
        http::middlewares::auth::auth,
        services::email_service::EmailService,
    },
};

pub fn router(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawler_repository = BrawlerPostgres::new(db_pool);
    let email_service = Arc::new(EmailService::new());
    let brawlers_use_case = Arc::new(BrawlersUseCase::new(
        Arc::new(brawler_repository),
        email_service
    ));

    Router::new()
        .route("/register", post(register))
        .route("/avatar", post(upload_avatar).layer(middleware::from_fn(auth)))
        .route("/update-name", post(update_display_name).layer(middleware::from_fn(auth)))
        .with_state(brawlers_use_case)
}

pub async fn register(
    State(use_case): State<Arc<BrawlersUseCase<BrawlerPostgres>>>,
    Json(payload): Json<RegisterBrawlerModel>,
) -> impl IntoResponse {
    match use_case.register(payload).await {
        Ok(user_id) => (StatusCode::CREATED, user_id.to_string()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn upload_avatar(
    State(use_case): State<Arc<BrawlersUseCase<BrawlerPostgres>>>,
    Extension(user_id): Extension<i32>,
    Json(payload): Json<AvatarUploadRequest>,
) -> impl IntoResponse {
    match use_case.upload_avatar(user_id, payload.base64_string).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn update_display_name(
    State(use_case): State<Arc<BrawlersUseCase<BrawlerPostgres>>>,
    Extension(user_id): Extension<i32>,
    Json(payload): Json<UpdateDisplayNameRequest>,
) -> impl IntoResponse {
    match use_case.update_display_name(user_id, payload.display_name).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
