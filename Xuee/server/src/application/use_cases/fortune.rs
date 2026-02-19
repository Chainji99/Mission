use std::sync::Arc;
use anyhow::Result;
use chrono::Utc;

use crate::domain::entities::fortune::{DailyFortuneDetail, NewDailyFortuneEntity, FortuneStickEntity};
use crate::domain::repositories::fortune::FortuneRepository;

pub struct FortuneUseCase<T>
where
    T: FortuneRepository + Send + Sync,
{
    fortune_repository: Arc<T>,
}

impl<T> FortuneUseCase<T>
where
    T: FortuneRepository + Send + Sync,
{
    pub fn new(fortune_repository: Arc<T>) -> Self {
        Self { fortune_repository }
    }

    pub async fn get_anonymous_fortune(&self) -> Result<FortuneStickEntity> {
        let stick = self.fortune_repository.get_random_stick().await?;
        Ok(stick)
    }

    pub async fn get_my_daily_fortune(&self, user_id: i32) -> Result<DailyFortuneDetail> {
        let today = Utc::now().date_naive();
        
        // Check if already drawn today
        if let Some(existing) = self.fortune_repository.get_daily_fortune(user_id, today).await? {
            return Ok(existing);
        }
        
        // If not, draw a new random stick
        let stick = self.fortune_repository.get_random_stick().await?;
        
        let new_daily = NewDailyFortuneEntity {
            user_id,
            stick_id: stick.id,
            date: today,
        };
        
        let created_fortune = self.fortune_repository.create_daily_fortune(new_daily).await?;
        
        Ok(DailyFortuneDetail {
            daily_fortune: created_fortune,
            stick,
        })
    }
}
