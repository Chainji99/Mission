use anyhow::Result;

pub struct MissionViewingRepository;

impl MissionViewingRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_one(&self, _mission_id: i32) -> Result<()> {
        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<()>> {
        Ok(vec![])
    }

    pub async fn crew_counting(&self, _mission_id: i32) -> Result<u32> {
        Ok(0)
    }
}
