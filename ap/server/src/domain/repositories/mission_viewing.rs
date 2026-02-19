use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait MissionViewingRepository {
    async fn crew_counting(&self, mission_id: i32) -> Result<u32>;
    async fn get_one(&self, mission_id: i32) -> Result<()>;
    async fn get_all(&self, filter: &()) -> Result<Vec<()>>;
    async fn get_crew(&self, filter: &()) -> Result<Vec<()>>;
}