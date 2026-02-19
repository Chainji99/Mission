use std::sync::Arc;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{post, put, delete},
    Json, Router,
    middleware,
};

use crate::{
    application::use_cases::mission_management::MissionManagementUseCase,
    domain::{
        value_objects::mission_model::{AddMissionModel, EditMissionModel},
        repositories::{
            mission_management::MissionManagementRepository,
            mission_viewing::MissionViewingRepository,
        },
    },
    infrastructure::{
        database::{
            repositories::{
                mission_management::MisssionManagementPostgres,
                mission_viewing::MissionViewingPostgres,
            },
            postgresql_connection::PgPoolSquad,
        },
        http::middlewares::auth::auth,
    },
};

pub fn router(db_pool: Arc<PgPoolSquad>) -> Router {
    let mission_management_repository = Arc::new(MisssionManagementPostgres::new(db_pool.clone()));
    let mission_viewing_repository = Arc::new(MissionViewingPostgres::new(db_pool));
    
    let use_case = Arc::new(MissionManagementUseCase::new(
        mission_management_repository,
        mission_viewing_repository,
    ));

    Router::new()
        .route("/", post(add))
        .route("/:id", put(edit))
        .route("/:id", delete(remove))
        .layer(middleware::from_fn(auth))
        .with_state(use_case)
}

async fn add<T1, T2>(
    State(use_case): State<Arc<MissionManagementUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Json(body): Json<AddMissionModel>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match use_case.add(user_id, body).await {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({ "mission_id": id }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn edit<T1, T2>(
    State(use_case): State<Arc<MissionManagementUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(id): Path<i32>,
    Json(body): Json<EditMissionModel>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match use_case.edit(id, user_id, body).await {
        Ok(id) => (StatusCode::OK, Json(serde_json::json!({ "mission_id": id }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn remove<T1, T2>(
    State(use_case): State<Arc<MissionManagementUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match use_case.remove(id, user_id).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "message": "Mission removed" }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}