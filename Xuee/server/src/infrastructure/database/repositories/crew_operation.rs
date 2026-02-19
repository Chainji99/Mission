use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::{insert_into, delete};
use diesel_async::RunQueryDsl;

use crate::domain::{
    entities::crew_memberships::CrewMemberShips,
    repositories::crew_operation::CrewOperationRepository,
};
use crate::infrastructure::database::{
    postgresql_connection::PgPoolSquad,
    schema::crew_memberships,
};

pub struct CrewOperationPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewOperationPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewOperationRepository for CrewOperationPostgres {
    async fn join(&self, crew_membership: CrewMemberShips) -> Result<()> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        insert_into(crew_memberships::table)
            .values(&crew_membership)
            .execute(&mut connection)
            .await?;

        Ok(())
    }

    async fn leave(&self, crew_membership: CrewMemberShips) -> Result<()> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_membership.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_membership.mission_id))
            .execute(&mut connection)
            .await?;

        Ok(())
    }
}
