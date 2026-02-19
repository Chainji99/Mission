use std::sync::Arc;
use async_trait::async_trait;
use diesel::{insert_into, QueryDsl, ExpressionMethods, PgTextExpressionMethods};
use diesel_async::RunQueryDsl;
use anyhow::Result;
use std::collections::HashMap;

use crate::domain::{
    entities::missions::{AddMissionEntity, EditMissionEntity, MissionEntity},
    repositories::missions::MissionRepository,
    value_objects::mission_filter::MissionFilter,
};
use crate::infrastructure::database::{
    postgresql_connection::PgPoolSquad,
    schema::{missions, crew_memberships},
};

pub struct MissionPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionRepository for MissionPostgres {
    async fn create(&self, new_mission: AddMissionEntity) -> Result<i32> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let result = insert_into(missions::table)
            .values(&new_mission)
            .returning(missions::id)
            .get_result(&mut connection)
            .await?;
        
        Ok(result)
    }

    async fn find_by_id(&self, id: i32) -> Result<MissionEntity> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let result = missions::table
            .find(id)
            .first(&mut connection)
            .await?;
        
        Ok(result)
    }

    async fn find_all(&self, filter: MissionFilter) -> Result<Vec<(MissionEntity, i64)>> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        // 1. Fetch Missions with Filters
        let mut query = missions::table.into_boxed();

        if let Some(name) = filter.name {
            query = query.filter(missions::name.ilike(format!("%{}%", name)));
        }

        if let Some(status) = filter.status {
            query = query.filter(missions::status.eq(status.to_string()));
        }

        let missions_list = query.load::<MissionEntity>(&mut connection).await?;
        
        if missions_list.is_empty() {
            return Ok(vec![]);
        }

        // 2. Fetch Crew Counts
        let mission_ids: Vec<i32> = missions_list.iter().map(|m| m.id).collect();
        
        let counts: Vec<(i32, i64)> = crew_memberships::table
            .filter(crew_memberships::mission_id.eq_any(&mission_ids))
            .group_by(crew_memberships::mission_id)
            .select((crew_memberships::mission_id, diesel::dsl::count(crew_memberships::brawler_id)))
            .load::<(i32, i64)>(&mut connection)
            .await?;

        let counts_map: HashMap<i32, i64> = counts.into_iter().collect();

        // 3. Merge
        let results = missions_list.into_iter().map(|m| {
            let count = counts_map.get(&m.id).copied().unwrap_or(0);
            (m, count)
        }).collect();

        Ok(results)
    }

    async fn update(&self, id: i32, mission: EditMissionEntity) -> Result<()> {
         let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;
         
         diesel::update(missions::table.find(id))
            .set(&mission)
            .execute(&mut connection)
            .await?;
            
         Ok(())
    }

    async fn delete(&self, id: i32) -> Result<()> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        diesel::update(missions::table.find(id))
            .set(missions::deleted_at.eq(chrono::Utc::now().naive_utc()))
            .execute(&mut connection)
            .await?;
            
        Ok(())
    }

    async fn join(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        insert_into(crew_memberships::table)
            .values((
                crew_memberships::mission_id.eq(mission_id),
                crew_memberships::brawler_id.eq(brawler_id),
                crew_memberships::joined_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut connection)
            .await?;
            
        Ok(())
    }
}
