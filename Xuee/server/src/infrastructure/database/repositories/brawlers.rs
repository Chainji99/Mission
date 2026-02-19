use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::{insert_into, update, QueryDsl};
use diesel::sql_types::Int4;
use diesel_async::RunQueryDsl;

use crate::domain::{
    entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
    repositories::brawlers::BrawlerRepository,
    value_objects::mission_model::MissionModel,
};
use crate::infrastructure::database::{
    postgresql_connection::PgPoolSquad,
    schema::brawlers,
};

pub struct BrawlerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BrawlerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BrawlerRepository for BrawlerPostgres {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let result = insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)
            .await?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let result = brawlers::table
            .filter(brawlers::username.eq(username))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)
            .await?;

        Ok(result)
    }

    async fn update_avatar(&self, id: i32, avatar_url: String) -> Result<()> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        update(brawlers::table.filter(brawlers::id.eq(id)))
            .set(brawlers::avatar_url.eq(avatar_url))
            .execute(&mut connection)
            .await?;

        Ok(())
    }

    async fn update_display_name(&self, id: i32, display_name: String) -> Result<()> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        update(brawlers::table.filter(brawlers::id.eq(id)))
            .set(brawlers::display_name.eq(display_name))
            .execute(&mut connection)
            .await?;

        Ok(())
    }

    async fn update_password(&self, id: i32, password_hash: String) -> Result<()> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        update(brawlers::table.filter(brawlers::id.eq(id)))
            .set(brawlers::password.eq(password_hash))
            .execute(&mut connection)
            .await?;

        Ok(())
    }

    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>> {
        let mut conn = self.db_pool.get().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let sql = r#"
            SELECT 
                m.id, m.name, m.description, m.status, m.chief_id, 
                b.display_name as chief_display_name,
                (SELECT COUNT(*) FROM crew_memberships cm WHERE cm.mission_id = m.id) as crew_count,
                m.created_at, m.updated_at
            FROM missions m
            JOIN brawlers b ON m.chief_id = b.id
            JOIN crew_memberships cm ON cm.mission_id = m.id
            WHERE cm.brawler_id = $1
        "#;

        let results = diesel::sql_query(sql)
            .bind::<Int4, _>(brawler_id)
            .load::<MissionModel>(&mut conn)
            .await?;

        Ok(results)
    }
}
