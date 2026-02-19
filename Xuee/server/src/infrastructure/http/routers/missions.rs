use std::sync::Arc;
use axum::{
    extract::{Extension, Query, State, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
    middleware,
};

use crate::{
    application::use_cases::missions::MissionsUseCase,
    domain::value_objects::mission_filter::MissionFilter,
    infrastructure::{
        database::{
            repositories::{
                missions::MissionPostgres,
                mission_viewing::MissionViewingPostgres,
            },
            postgresql_connection::PgPoolSquad,
        },
        http::middlewares::auth::auth,
    },
};

pub fn router(db_pool: Arc<PgPoolSquad>) -> Router {
    let mission_repository = MissionPostgres::new(db_pool.clone());
    let mission_viewing_repository = MissionViewingPostgres::new(db_pool);
    let missions_use_case = Arc::new(MissionsUseCase::new(
        Arc::new(mission_repository),
        Arc::new(mission_viewing_repository),
    ));

    Router::new()
        .route("/", get(get_all))
        .route("/:id/join", post(join).layer(middleware::from_fn(auth)))
        .with_state(missions_use_case)
}

async fn get_all(
    State(use_case): State<Arc<MissionsUseCase<MissionPostgres, MissionViewingPostgres>>>,
    Query(filter): Query<MissionFilter>,
) -> impl IntoResponse {
    match use_case.get_all(filter).await {
        Ok(missions) => (StatusCode::OK, Json(missions)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn join(
    State(use_case): State<Arc<MissionsUseCase<MissionPostgres, MissionViewingPostgres>>>,
    Extension(user_id): Extension<i32>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match use_case.join(id, user_id).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"message": "Joined successfully"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
