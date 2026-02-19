use axum::Router;

pub fn router() -> Router {
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