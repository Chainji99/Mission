use std::sync::Arc;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::domain::entities::cards::{CardEntity, UserCardEntity, NewUserCardEntity, UserCardDetail};
use crate::domain::entities::battles::{BattleEntity, NewBattleEntity};
use crate::domain::repositories::cards::CardRepository;
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::infrastructure::database::schema::{cards, user_cards, battles};

pub struct CardPostgres {
    pool: Arc<PgPoolSquad>,
}

impl CardPostgres {
    pub fn new(pool: Arc<PgPoolSquad>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CardRepository for CardPostgres {
    async fn get_all_cards(&self) -> Result<Vec<CardEntity>> {
        let mut conn = self.pool.get().await?;
        let result = cards::table
            .load::<CardEntity>(&mut conn)
            .await?;
        Ok(result)
    }

    async fn get_user_cards(&self, user_id_val: i32) -> Result<Vec<UserCardDetail>> {
        let mut conn = self.pool.get().await?;
        
        let data = user_cards::table
            .inner_join(cards::table)
            .filter(user_cards::user_id.eq(user_id_val))
            .load::<(UserCardEntity, CardEntity)>(&mut conn)
            .await?;
            
        let details = data.into_iter().map(|(uc, c)| UserCardDetail {
            user_card: uc,
            card: c,
        }).collect();
        
        Ok(details)
    }

    async fn add_user_card(&self, new_card: NewUserCardEntity) -> Result<UserCardEntity> {
        let mut conn = self.pool.get().await?;
        let result = diesel::insert_into(user_cards::table)
            .values(&new_card)
            .get_result::<UserCardEntity>(&mut conn)
            .await?;
        Ok(result)
    }

    async fn update_card_exp(&self, user_card_id: i32, new_exp: i32, new_level: i32) -> Result<()> {
        let mut conn = self.pool.get().await?;
        diesel::update(user_cards::table.find(user_card_id))
            .set((
                user_cards::experience.eq(new_exp),
                user_cards::level.eq(new_level),
            ))
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    async fn get_card_by_id(&self, card_id_val: i32) -> Result<CardEntity> {
        let mut conn = self.pool.get().await?;
        let result = cards::table
            .find(card_id_val)
            .first::<CardEntity>(&mut conn)
            .await
            .map_err(|e| anyhow!("Card not found: {}", e))?;
        Ok(result)
    }

    async fn get_user_card_by_id(&self, user_card_id_val: i32) -> Result<UserCardDetail> {
        let mut conn = self.pool.get().await?;
        let (uc, c) = user_cards::table
            .find(user_card_id_val)
            .inner_join(cards::table)
            .first::<(UserCardEntity, CardEntity)>(&mut conn)
            .await
            .map_err(|e| anyhow!("User Card not found: {}", e))?;
            
        Ok(UserCardDetail { user_card: uc, card: c })
    }

    async fn create_battle(&self, battle: NewBattleEntity) -> Result<BattleEntity> {
        let mut conn = self.pool.get().await?;
        let result = diesel::insert_into(battles::table)
            .values(&battle)
            .get_result::<BattleEntity>(&mut conn)
            .await?;
        Ok(result)
    }
}
