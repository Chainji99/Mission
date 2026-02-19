use std::sync::Arc;
use anyhow::Result;

pub struct MissionOperationUseCase<T1, T2>
where
    T1: Send + Sync,
    T2: Send + Sync,
{
    mission_operation_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
}

impl<T1, T2> MissionOperationUseCase<T1, T2>
where
    T1: Send + Sync,
    T2: Send + Sync,
{
    pub fn new(mission_operation_repository: Arc<T1>, mission_viewing_repository: Arc<T2>) -> Self {
        Self {
            mission_operation_repository,
            mission_viewing_repository,
        }
    }

    pub async fn in_progress(&self, _mission_id: i32, _chief_id: i32) -> Result<i32> {
        Ok(0)
    }

    pub async fn to_completed(&self, _mission_id: i32, _chief_id: i32) -> Result<i32> {
        Ok(0)
    }

    pub async fn to_failed(&self, _mission_id: i32, _chief_id: i32) -> Result<i32> {
        Ok(0)
    }
}