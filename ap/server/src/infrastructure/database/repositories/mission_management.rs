use anyhow::Result;

pub struct MissionManagementRepository;

impl MissionManagementRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn add(&self, _chief_id: i32, _name: String) -> Result<i32> {
        Ok(1)
    }

    pub async fn edit(&self, _mission_id: i32, _name: String) -> Result<i32> {
        Ok(1)
    }

    pub async fn remove(&self, _mission_id: i32) -> Result<()> {
        Ok(())
    }
}
