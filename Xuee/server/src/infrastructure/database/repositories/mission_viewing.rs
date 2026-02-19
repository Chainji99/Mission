use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::sql_types::{Int4, Nullable, Varchar};
use diesel_async::RunQueryDsl;
use std::sync::Arc;

use crate::domain::value_objects::{
    mission_model::MissionModel,
    mission_filter::MissionFilter,
    brawler_model::BrawlerModel,
};
use crate::domain::repositories::mission_viewing::MissionViewingRepository;
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::infrastructure::database::schema::crew_memberships;

pub struct MissionViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionViewingRepository for MissionViewingPostgres {
    async fn view_detail(&self, mission_id: i32) -> Result<MissionModel> {
        let mut conn = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let sql = r#"
            SELECT 
                m.id, m.name, m.description, m.status, m.chief_id, 
                b.display_name as chief_display_name,
                (SELECT COUNT(*) FROM crew_memberships cm WHERE cm.mission_id = m.id) as crew_count,
                m.mission_date, m.time, m.email, m.phone, m.location, m.rewards,
                m.created_at, m.updated_at
            FROM missions m
            JOIN brawlers b ON m.chief_id = b.id
            WHERE m.id = $1
        "#;

        let result = diesel::sql_query(sql)
            .bind::<Int4, _>(mission_id)
            .get_result::<MissionModel>(&mut conn)
            .await?;

        Ok(result)
    }

    async fn gets(&self, filter: &MissionFilter) -> Result<Vec<MissionModel>> {
        let mut conn = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let sql = r#"
            SELECT 
                m.id, m.name, m.description, m.status, m.chief_id, 
                b.display_name as chief_display_name,
                (SELECT COUNT(*) FROM crew_memberships cm WHERE cm.mission_id = m.id) as crew_count,
                m.mission_date, m.time, m.email, m.phone, m.location, m.rewards,
                m.created_at, m.updated_at
            FROM missions m
            JOIN brawlers b ON m.chief_id = b.id
            WHERE ($1 IS NULL OR m.status = $1)
              AND ($2 IS NULL OR m.name ILIKE $2)
        "#;

        let status_bind = filter.status.as_ref().map(|s| s.to_string());
        let name_bind = filter.name.as_ref().map(|n| format!("%{}%", n));

        let rows = diesel::sql_query(sql)
            .bind::<Nullable<Varchar>, _>(status_bind)
            .bind::<Nullable<Varchar>, _>(name_bind)
            .load::<MissionModel>(&mut conn)
            .await?;

        Ok(rows)
    }

    async fn crew_counting(&self, mission_id: i32) -> Result<u32> {
        let mut conn = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        let count = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;
            
        Ok(count as u32)
    }

    async fn get_mission_crew(&self, mission_id: i32) -> Result<Vec<BrawlerModel>> {
        let mut conn = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let sql = r#"
            SELECT b.id, b.username, b.display_name 
            FROM brawlers b 
            JOIN crew_memberships cm ON b.id = cm.brawler_id 
            WHERE cm.mission_id = $1
        "#;

        let crew = diesel::sql_query(sql)
            .bind::<Int4, _>(mission_id)
            .load::<BrawlerModel>(&mut conn)
            .await?;

        Ok(crew)
    }
}
