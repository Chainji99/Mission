use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity};
use crate::domain::value_objects::mission_model::MissionModel;

#[async_trait]
#[automock]
pub trait BrawlerRepository {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity>;
    async fn update_avatar(&self, id: i32, avatar_url: String) -> Result<()>;
    async fn update_display_name(&self, id: i32, display_name: String) -> Result<()>;
    async fn update_password(&self, id: i32, password_hash: String) -> Result<()>;
    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>>;
}
