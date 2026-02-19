use std::sync::Arc;
use anyhow::{anyhow, Result};

use crate::domain::{
    repositories::{
        mission_operation::MissionOperationRepository,
        mission_viewing::MissionViewingRepository,
    },
    value_objects::mission_statuses::MissionStatuses,
};

pub struct MissionOperationUseCase<T1, T2>
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    mission_operation_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
}

impl<T1, T2> MissionOperationUseCase<T1, T2>
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    pub fn new(
        mission_operation_repository: Arc<T1>,
        mission_viewing_repository: Arc<T2>,
    ) -> Self {
        Self {
            mission_operation_repository,
            mission_viewing_repository,
        }
    }

    pub async fn in_progress(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self
            .mission_viewing_repository
            .view_detail(mission_id)
            .await?;

        // Check if user is chief
        if mission.chief_id != chief_id {
            return Err(anyhow!("Only the chief can change mission status"));
        }

        let is_status_open_or_fail = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::Failed.to_string();

        if !is_status_open_or_fail {
            return Err(anyhow!("Mission must be Open or Failed to start progress"));
        }

        let result = self
            .mission_operation_repository
            .in_progress(mission_id, chief_id)
            .await?;
            
        Ok(result)
    }

    pub async fn to_completed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self
            .mission_viewing_repository
            .view_detail(mission_id)
            .await?;

        if mission.chief_id != chief_id {
            return Err(anyhow!("Only the chief can change mission status"));
        }

        let update_condition = mission.status == MissionStatuses::InProgress.to_string();
        
        if !update_condition {
            return Err(anyhow!("Mission must be In Progress to complete"));
        }

        let result = self
            .mission_operation_repository
            .to_completed(mission_id, chief_id)
            .await?;

        Ok(result)
    }

    pub async fn to_failed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self
            .mission_viewing_repository
            .view_detail(mission_id)
            .await?;

        if mission.chief_id != chief_id {
            return Err(anyhow!("Only the chief can change mission status"));
        }

        let update_condition = mission.status == MissionStatuses::InProgress.to_string();
        
        if !update_condition {
            return Err(anyhow!("Mission must be In Progress to fail"));
        }

        let result = self
            .mission_operation_repository
            .to_failed(mission_id, chief_id)
            .await?;

        Ok(result)
    }
}
