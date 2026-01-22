pub routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let mission_viewing_repository = Arc::new(MissionViewingPostgres::new(db_pool.clone()));
    let mission_viewing_use_case =
        Arc::new(MissionViewingUseCase::new(mission_viewing_repository));

    Router::new()
        .route("/missions/:mission_id/crew",get(mission_viewing_handler::get_crew::<MissionViewingUseCase<MissionViewingPostgres>>),>),
        .route("/filter", get(mission_viewing_handler::get_crew::<MissionViewingUseCase<MissionViewingPostgres>>),)
        .route("/crew", get(mission_viewing_handler::get_crew::<MissionViewingUseCase<MissionViewingPostgres>>),)
        // .route("/missions/:mission_id/crew", get(mission_viewing_handler::get_crew::<MissionViewingUseCase<MissionViewingPostgres>>),)
        .with_stats(Arc::clone(&mission_viewing_use_case))
    }