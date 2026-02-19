use std::sync::Arc;
use async_trait::async_trait;
use diesel::prelude::*;
use anyhow::Result;

use crate::{
    domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        repositories::brawlers::BrawlerRepository,
    },
    infrastructure::database::{postgresql_connection::PgPoolSquad, schema::brawlers},
};

pub struct PostgresBrawlerRepository {
    db_pool: Arc<PgPoolSquad>,
}

impl PostgresBrawlerRepository {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BrawlerRepository for PostgresBrawlerRepository {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32> {
        let mut connection = self.db_pool.get()?;

        let result = diesel::insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity> {
        let mut connection = self.db_pool.get()?;

        let result = brawlers::table
            .filter(brawlers::username.eq(username))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn upload_base64img(
        &self,
        user_id: i32,
        _base64_image: String,
        _opt: Option<()>,
    ) -> Result<String> {
        Ok(format!("avatar_{}", user_id))
    }
}
