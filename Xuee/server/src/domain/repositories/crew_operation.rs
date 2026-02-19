use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::entities::crew_memberships::CrewMemberShips;

#[async_trait]
#[automock]
pub trait CrewOperationRepository {
    async fn join(&self, crew_memberships: CrewMemberShips) -> Result<()>;
    async fn leave(&self, crew_memberships: CrewMemberShips) -> Result<()>;
}
