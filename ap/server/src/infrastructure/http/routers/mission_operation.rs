use std::sync::Arc;

use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    routing::post,
    Json, Router,
};

pub async fn to_progress<T1, T2>(
    State(mission_operation_use_case): State<Arc<MissionOperationUseCase<T1, T2>>>,
    Extension(chief_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match mission_operation_use_case.to_progress(mission_id, chief_id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Mission status updated to progress"}))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": format!("{}", e)}))),
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
    match mission_operation_use_case.to_progress(mission_id, chief_id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Mission status updated to progress"}))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": format!("{}", e)}))),
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
    match mission_operation_use_case.to_progress(mission_id, chief_id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Mission status updated to progress"}))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": format!("{}", e)}))),
    }
}

pub fn routes(db_pool: Arc<SqlitePool>) -> Router {
    let mission_repository = Arc::new(SqliteMissionOperationRepository::new(db_pool.clone()));
    let viewing_repository = Arc::new(SqliteMissionViewingRepository::new(db_pool.clone()));
    let mission_operation_use_case =
        Arc::new(MissionOperationUseCase::new(mission_repository, viewing_repository
    ));

    Router::new()
        .route(
            "/missions/:mission_id/to_progress",
            post(to_progress::<
                SqliteMissionOperationRepository,
                SqliteMissionViewingRepository,
            >),
        )
        .route(
            "/missions/:mission_id/to_complete",
            post(to_complete::<
                SqliteMissionOperationRepository,
                SqliteMissionViewingRepository,
            >),
        )
        .route(
            "/missions/:mission_id/to_failed",
            post(to_failed::<
                SqliteMissionOperationRepository,
                SqliteMissionViewingRepository,
            >),
        )
        .with_state(mission_operation_use_case)
}