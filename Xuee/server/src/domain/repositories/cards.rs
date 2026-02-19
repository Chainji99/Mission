use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use crate::domain::entities::cards::{CardEntity, UserCardEntity, NewUserCardEntity, UserCardDetail};
use crate::domain::entities::battles::{BattleEntity, NewBattleEntity};

#[async_trait]
#[automock]
pub trait CardRepository {
    async fn get_all_cards(&self) -> Result<Vec<CardEntity>>;
    async fn get_user_cards(&self, user_id: i32) -> Result<Vec<UserCardDetail>>;
    async fn add_user_card(&self, new_card: NewUserCardEntity) -> Result<UserCardEntity>;
    async fn update_card_exp(&self, user_card_id: i32, new_exp: i32, new_level: i32) -> Result<()>;
    async fn get_card_by_id(&self, card_id: i32) -> Result<CardEntity>;
    async fn get_user_card_by_id(&self, user_card_id: i32) -> Result<UserCardDetail>;
    
    // Battle related
    async fn create_battle(&self, battle: NewBattleEntity) -> Result<BattleEntity>;
}
