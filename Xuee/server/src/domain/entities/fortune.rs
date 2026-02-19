use serde::{Deserialize, Serialize};
use diesel::prelude::*;

// --- Fortune Stick ---
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::infrastructure::database::schema::fortune_sticks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FortuneStickEntity {
    pub id: i32,
    pub number: i32,
    pub poem_text: String,
    pub interpretation: String,
    pub lucky_direction: Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::infrastructure::database::schema::fortune_sticks)]
pub struct NewFortuneStickEntity {
    pub number: i32,
    pub poem_text: String,
    pub interpretation: String,
    pub lucky_direction: Option<String>,
}

// --- Daily Fortune ---
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::infrastructure::database::schema::daily_fortunes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DailyFortuneEntity {
    pub id: i32,
    pub user_id: i32,
    pub stick_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub date: chrono::NaiveDate,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::infrastructure::database::schema::daily_fortunes)]
pub struct NewDailyFortuneEntity {
    pub user_id: i32,
    pub stick_id: i32,
    pub date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyFortuneDetail {
    pub daily_fortune: DailyFortuneEntity,
    pub stick: FortuneStickEntity,
}
