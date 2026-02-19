use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use crate::domain::entities::fortune::{FortuneStickEntity, DailyFortuneEntity, NewDailyFortuneEntity, DailyFortuneDetail};

#[async_trait]
#[automock]
pub trait FortuneRepository {
    async fn get_stick_by_id(&self, stick_id: i32) -> Result<FortuneStickEntity>;
    async fn get_random_stick(&self) -> Result<FortuneStickEntity>;
    async fn get_daily_fortune(&self, user_id: i32, date: chrono::NaiveDate) -> Result<Option<DailyFortuneDetail>>;
    async fn create_daily_fortune(&self, new_fortune: NewDailyFortuneEntity) -> Result<DailyFortuneEntity>;
}
