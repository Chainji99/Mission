use crate::infrastructure::database::schema::{cards, user_cards};
use chrono::NaiveDateTime;
use diesel::{Selectable, Queryable, Identifiable, Insertable, Associations};
use crate::domain::entities::brawlers::BrawlerEntity;
use serde::Serialize;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Serialize)]
#[diesel(table_name = cards)]
pub struct CardEntity {
    pub id: i32,
    pub name: String,
    pub language: String,
    pub rarity: String,
    pub attack: i32,
    pub defense: i32,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Associations, Serialize)]
#[diesel(belongs_to(CardEntity, foreign_key = card_id))]
#[diesel(belongs_to(BrawlerEntity, foreign_key = user_id))]
#[diesel(table_name = user_cards)]
pub struct UserCardEntity {
    pub id: i32,
    pub user_id: i32,
    pub card_id: i32,
    pub level: i32,
    pub experience: i32,
    pub obtained_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = user_cards)]
pub struct NewUserCardEntity {
    pub user_id: i32,
    pub card_id: i32,
}

// Join struct for API response
#[derive(Debug, Serialize)]
pub struct UserCardDetail {
    pub user_card: UserCardEntity,
    pub card: CardEntity,
}
