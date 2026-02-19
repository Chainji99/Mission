use anyhow::Result;

pub struct MissionOperationRepository;

impl MissionOperationRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn in_progress(&self, _mission_id: i32) -> Result<i32> {
        Ok(1)
    }

    pub async fn to_completed(&self, _mission_id: i32) -> Result<i32> {
        Ok(1)
    }

    pub async fn to_failed(&self, _mission_id: i32) -> Result<i32> {
        Ok(1)
    }
}
