use async_trait::async_trait;
use mockall::automock;
use anyhow::Result;

use crate::domain::{
    entities::missions::{AddMissionEntity, EditMissionEntity, MissionEntity},
    value_objects::mission_filter::MissionFilter,
};

#[async_trait]
#[automock]
pub trait MissionRepository {
    async fn create(&self, mission: AddMissionEntity) -> Result<i32>;
    async fn find_by_id(&self, id: i32) -> Result<MissionEntity>;
    async fn find_all(&self, filter: MissionFilter) -> Result<Vec<(MissionEntity, i64)>>;
    async fn update(&self, id: i32, mission: EditMissionEntity) -> Result<()>;
    async fn delete(&self, id: i32) -> Result<()>;
    async fn join(&self, mission_id: i32, brawler_id: i32) -> Result<()>;
}
