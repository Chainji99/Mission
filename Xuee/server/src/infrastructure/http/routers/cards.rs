use std::sync::Arc;
use axum::{
    extract::{State, Extension},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
    middleware,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpgradeRequest {
    pub user_card_id: i32,
    pub amount: i32,
}

use crate::{
    application::use_cases::cards::CardUseCase,
    infrastructure::{
        database::{
            repositories::cards::CardPostgres,
            postgresql_connection::PgPoolSquad,
        },
        http::middlewares::auth::auth,
    },
};

pub fn router(db_pool: Arc<PgPoolSquad>) -> Router {
    let card_repository = CardPostgres::new(db_pool);
    let card_use_case = Arc::new(CardUseCase::new(Arc::new(card_repository)));

    Router::new()
        .route("/", get(get_all_cards))
        .route("/inventory", get(get_my_inventory).layer(middleware::from_fn(auth)))
        .route("/gacha", post(draw_gacha).layer(middleware::from_fn(auth)))
        .route("/upgrade", post(upgrade_card).layer(middleware::from_fn(auth)))
        .route("/battle", post(battle).layer(middleware::from_fn(auth)))
        .with_state(card_use_case)
}

pub async fn get_all_cards(
    State(use_case): State<Arc<CardUseCase<CardPostgres>>>,
) -> impl IntoResponse {
    match use_case.get_all_cards().await {
        Ok(cards) => (StatusCode::OK, Json(cards)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_my_inventory(
    State(use_case): State<Arc<CardUseCase<CardPostgres>>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse {
    match use_case.get_user_inventory(user_id).await {
        Ok(cards) => (StatusCode::OK, Json(cards)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[axum::debug_handler]
pub async fn draw_gacha(
    State(use_case): State<Arc<CardUseCase<CardPostgres>>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse {
    match use_case.draw_gacha(user_id).await {
        Ok(card) => (StatusCode::CREATED, Json(card)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn upgrade_card(
    State(use_case): State<Arc<CardUseCase<CardPostgres>>>,
    Extension(_user_id): Extension<i32>,
    Json(payload): Json<UpgradeRequest>,
) -> impl IntoResponse {
    // Note: In real app, verify user_id owns the card
    match use_case.upgrade_card(payload.user_card_id, payload.amount).await {
        Ok(msg) => (StatusCode::OK, Json(serde_json::json!({ "message": msg }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
pub struct BattleRequest {
    pub defender_id: i32,
}

pub async fn battle(
    State(use_case): State<Arc<CardUseCase<CardPostgres>>>,
    Extension(user_id): Extension<i32>,
    Json(payload): Json<BattleRequest>,
) -> impl IntoResponse {
    match use_case.battle(user_id, payload.defender_id).await {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
