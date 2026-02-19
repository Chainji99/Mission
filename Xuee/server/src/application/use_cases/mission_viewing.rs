use std::sync::Arc;
use anyhow::Result;

use crate::domain::repositories::mission_viewing::MissionViewingRepository;
use crate::domain::value_objects::{mission_filter::MissionFilter, mission_model::MissionModel};

pub struct MissionViewingUseCase<T>
where
    T: MissionViewingRepository + Send + Sync,
{
    mission_viewing_repository: Arc<T>,
}

impl<T> MissionViewingUseCase<T>
where
    T: MissionViewingRepository + Send + Sync,
{
    pub fn new(mission_viewing_repository: Arc<T>) -> Self {
        Self {
            mission_viewing_repository,
        }
    }

    pub async fn get_all(&self, filter: &MissionFilter) -> Result<Vec<MissionModel>> {
        self.mission_viewing_repository.gets(filter).await
    }

    pub async fn get_one(&self, mission_id: i32) -> Result<MissionModel> {
        self.mission_viewing_repository.view_detail(mission_id).await
    }
}
