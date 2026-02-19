use anyhow::Result;

pub struct CrewOperationRepository;

impl CrewOperationRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn join(&self, _mission_id: i32, _brawler_id: i32) -> Result<()> {
        Ok(())
    }

    pub async fn leave(&self, _mission_id: i32, _brawler_id: i32) -> Result<()> {
        Ok(())
    }
}