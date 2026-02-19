use crate::infrastructure::database::schema::battles;
use chrono::NaiveDateTime;
use diesel::{Selectable, Queryable, Identifiable, Insertable};
use serde::Serialize;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Serialize)]
#[diesel(table_name = battles)]
pub struct BattleEntity {
    pub id: i32,
    pub attacker_id: i32,
    pub defender_id: i32,
    pub winner_id: Option<i32>,
    pub log: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = battles)]
pub struct NewBattleEntity {
    pub attacker_id: i32,
    pub defender_id: i32,
    pub winner_id: Option<i32>,
    pub log: Option<String>,
}
