use std::sync::Arc;
use axum::{
    extract::{State, Extension},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
    middleware,
};

use crate::{
    application::use_cases::fortune::FortuneUseCase,
    infrastructure::{
        database::{
            repositories::fortune::FortunePostgres,
            postgresql_connection::PgPoolSquad,
        },
        http::middlewares::auth::auth,
    },
};

pub fn router(db_pool: Arc<PgPoolSquad>) -> Router {
    let fortune_repository = FortunePostgres::new(db_pool);
    let fortune_use_case = Arc::new(FortuneUseCase::new(Arc::new(fortune_repository)));

    Router::new()
        .route("/daily", get(get_daily_fortune).layer(middleware::from_fn(auth)))
        .route("/draw", get(draw_fortune))
        .with_state(fortune_use_case)
}

pub async fn draw_fortune(
    State(use_case): State<Arc<FortuneUseCase<FortunePostgres>>>,
) -> impl IntoResponse {
    match use_case.get_anonymous_fortune().await {
        Ok(stick) => (StatusCode::OK, Json(stick)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_daily_fortune(
    State(use_case): State<Arc<FortuneUseCase<FortunePostgres>>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse {
    match use_case.get_my_daily_fortune(user_id).await {
        Ok(detail) => (StatusCode::OK, Json(detail)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
