use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use crate::{
    domain::{
        entities::{
            brawlers::RegisterBrawlerEntity,
            missions::AddMissionEntity,
        },
        repositories::{
            brawlers::BrawlerRepository,
            missions::MissionRepository,
        },
        value_objects::mission_statuses::MissionStatuses,
    },
    infrastructure::database::{
        repositories::{
            brawlers::BrawlerPostgres,
            missions::MissionPostgres,
        },
        postgresql_connection::PgPoolSquad,
    },
};

pub fn router(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawler_repository = Arc::new(BrawlerPostgres::new(db_pool.clone()));
    let mission_repository = Arc::new(MissionPostgres::new(db_pool));

    Router::new()
        .route("/seed", post(seed))
        .with_state((brawler_repository, mission_repository))
}

#[axum::debug_handler]
pub async fn seed(
    State((brawler_repo, mission_repo)): State<(Arc<BrawlerPostgres>, Arc<MissionPostgres>)>,
) -> impl IntoResponse {
    // 1. Create 10 Users
    let mut user_ids = Vec::new();
    let names = vec![
        ("zen_master", "Zen Master"),
        ("silent_blade", "Silent Blade"),
        ("cherry_blossom", "Cherry Blossom"),
        ("mountain_monk", "Mountain Monk"),
        ("dragon_spirit", "Dragon Spirit"),
        ("wind_walker", "Wind Walker"),
        ("moon_shadow", "Moon Shadow"),
        ("fire_fox", "Fire Fox"),
        ("river_stone", "River Stone"),
        ("golden_crane", "Golden Crane"),
    ];

    for (username, display_name) in names {
        let entity = RegisterBrawlerEntity {
            username: username.to_string(),
            password: "$argon2id$v=19$m=19456,t=2,p=1$VE0eXwFzCjI$P1/o7tS9q+k".to_string(), // Dummy hash for "password"
            display_name: display_name.to_string(),
        };
        
        if let Ok(id) = brawler_repo.register(entity).await {
            user_ids.push(id);
        } else {
            if let Ok(user) = brawler_repo.find_by_username(username.to_string()).await {
                user_ids.push(user.id);
            }
        }
    }

    if user_ids.is_empty() {
         return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create users").into_response();
    }

    // 2. Create 2 Missions of EACH status (Total 8)
    let statuses = vec![
        MissionStatuses::Open,
        MissionStatuses::InProgress,
        MissionStatuses::Completed,
        MissionStatuses::Failed,
    ];

    let mut created_missions = 0;
    let mut total_crew = 0;
    
    for status in statuses {
        for i in 1..=2 {
            // Distribute chiefs among missions
            let chief_index = (created_missions % user_ids.len()) as usize;
            let chief_id = user_ids[chief_index];
            
            let entity = AddMissionEntity {
                chief_id,
                name: format!("{} Mission {}", status, i),
                description: Some(format!("This is a mock mission for status {} (Iteration {})", status, i)),
                status: status.to_string(),
                mission_date: None,
                time: None,
                email: None,
                phone: None,
                location: None,
                rewards: None,
            };
            
            if let Ok(mission_id) = mission_repo.create(entity).await {
                created_missions += 1;
                
                // Add 2 crew members for each mission
                let mut crew_added = 0;
                let mut user_index = 0;
                
                while crew_added < 2 && user_index < user_ids.len() {
                    let brawler_id = user_ids[user_index];
                    if brawler_id != chief_id {
                        if mission_repo.join(mission_id, brawler_id).await.is_ok() {
                            crew_added += 1;
                            total_crew += 1;
                        }
                    }
                    user_index += 1;
                }
            }
        }
    }

    (StatusCode::OK, format!("Seeded {} users, {} missions, and {} crew members (2 missions per status, 2 crew per mission)", user_ids.len(), created_missions, total_crew)).into_response()
}
