use std::sync::Arc;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::domain::entities::fortune::{FortuneStickEntity, DailyFortuneEntity, NewDailyFortuneEntity, DailyFortuneDetail};
use crate::domain::repositories::fortune::FortuneRepository;
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::infrastructure::database::schema::{fortune_sticks, daily_fortunes};

pub struct FortunePostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl FortunePostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl FortuneRepository for FortunePostgres {
    async fn get_stick_by_id(&self, stick_id: i32) -> Result<FortuneStickEntity> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow!(e.to_string()))?;
        
        let stick = fortune_sticks::table
            .find(stick_id)
            .first(&mut connection)
            .await?;
            
        Ok(stick)
    }

    async fn get_random_stick(&self) -> Result<FortuneStickEntity> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow!(e.to_string()))?;
        
        // Use SQL random function
        let stick = fortune_sticks::table
            .order(diesel::dsl::sql::<diesel::sql_types::Float>("RANDOM()"))
            .first(&mut connection)
            .await?;
            
        Ok(stick)
    }

    async fn get_daily_fortune(&self, user_id_val: i32, date_val: chrono::NaiveDate) -> Result<Option<DailyFortuneDetail>> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow!(e.to_string()))?;
        
        let result: Option<(DailyFortuneEntity, FortuneStickEntity)> = daily_fortunes::table
            .inner_join(fortune_sticks::table)
            .filter(daily_fortunes::user_id.eq(user_id_val))
            .filter(daily_fortunes::date.eq(date_val))
            .first(&mut connection)
            .await
            .optional()?;
            
        Ok(result.map(|(df, fs)| DailyFortuneDetail {
            daily_fortune: df,
            stick: fs,
        }))
    }

    async fn create_daily_fortune(&self, new_fortune: NewDailyFortuneEntity) -> Result<DailyFortuneEntity> {
        let mut connection = self.db_pool.get().await.map_err(|e| anyhow!(e.to_string()))?;
        
        let created = diesel::insert_into(daily_fortunes::table)
            .values(&new_fortune)
            .get_result(&mut connection)
            .await?;
            
        Ok(created)
    }
}
