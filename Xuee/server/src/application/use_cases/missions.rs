use std::sync::Arc;
use anyhow::Result;

use crate::domain::{
    repositories::{
        missions::MissionRepository,
        mission_viewing::MissionViewingRepository,
    },
    value_objects::{
        mission_filter::MissionFilter,
        mission_model::{AddMissionModel, MissionModel},
    },
};

pub struct MissionsUseCase<T1, T2>
where
    T1: MissionRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    repository: Arc<T1>,
    viewing_repository: Arc<T2>,
}

impl<T1, T2> MissionsUseCase<T1, T2>
where
    T1: MissionRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    pub fn new(repository: Arc<T1>, viewing_repository: Arc<T2>) -> Self {
        Self { repository, viewing_repository }
    }

    pub async fn create(&self, chief_id: i32, model: AddMissionModel) -> Result<i32> {
        self.repository.create(model.to_entity(chief_id)).await
    }

    pub async fn get_all(&self, filter: MissionFilter) -> Result<Vec<MissionModel>> {
        self.viewing_repository.gets(&filter).await
    }

    pub async fn join(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        self.repository.join(mission_id, brawler_id).await
    }
}
