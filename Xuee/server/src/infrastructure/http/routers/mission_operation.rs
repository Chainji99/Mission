use std::sync::Arc;
use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    routing::post,
    Json, Router, http::StatusCode,
};
use serde_json::json;

use crate::{
    application::use_cases::mission_operation::MissionOperationUseCase,
    domain::repositories::{
        mission_operation::MissionOperationRepository,
        mission_viewing::MissionViewingRepository,
    },
    infrastructure::database::{
        repositories::{
            mission_operation::MissionOperationPostgres,
            mission_viewing::MissionViewingPostgres,
        },
        postgresql_connection::PgPoolSquad,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let mission_repository = Arc::new(MissionOperationPostgres::new(db_pool.clone()));
    let viewing_repository = Arc::new(MissionViewingPostgres::new(db_pool.clone()));
    let mission_operation_use_case =
        Arc::new(MissionOperationUseCase::new(mission_repository, viewing_repository));

    Router::new()
        .route(
            "/missions/:mission_id/to_progress",
            post(to_progress::<
                MissionOperationPostgres,
                MissionViewingPostgres,
            >),
        )
        .route(
            "/missions/:mission_id/to_complete",
            post(to_complete::<
                MissionOperationPostgres,
                MissionViewingPostgres,
            >),
        )
        .route(
            "/missions/:mission_id/to_failed",
            post(to_failed::<
                MissionOperationPostgres,
                MissionViewingPostgres,
            >),
        )
        .with_state(mission_operation_use_case)
}

pub async fn to_progress<T1, T2>(
    State(mission_operation_use_case): State<Arc<MissionOperationUseCase<T1, T2>>>,
    Extension(chief_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match mission_operation_use_case.in_progress(mission_id, chief_id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Mission status updated to progress"}))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": format!("{}", e)}))).into_response(),
    }
}


pub async fn to_complete<T1, T2>(
    State(mission_operation_use_case): State<Arc<MissionOperationUseCase<T1, T2>>>,
    Extension(chief_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match mission_operation_use_case.to_completed(mission_id, chief_id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Mission status updated to completed"}))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": format!("{}", e)}))).into_response(),
    }
}

pub async fn to_failed<T1, T2>(
    State(mission_operation_use_case): State<Arc<MissionOperationUseCase<T1, T2>>>,
    Extension(chief_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match mission_operation_use_case.to_failed(mission_id, chief_id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Mission status updated to failed"}))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": format!("{}", e)}))).into_response(),
    }
}
