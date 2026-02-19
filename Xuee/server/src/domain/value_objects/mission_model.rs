use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::{
    QueryableByName,
    sql_types::{BigInt, Int4, Nullable, Text, Timestamp, Varchar},
};
use crate::domain::{
    entities::missions::{AddMissionEntity, EditMissionEntity, MissionEntity},
    value_objects::mission_statuses::MissionStatuses,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, QueryableByName)]
pub struct MissionModel {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub description: Option<String>,
    #[diesel(sql_type = Varchar)]
    pub status: String,
    #[diesel(sql_type = Int4)]
    pub chief_id: i32,
    #[diesel(sql_type = Varchar)]
    pub chief_display_name: String,
    #[diesel(sql_type = BigInt)]
    pub crew_count: i64,
    #[diesel(sql_type = Nullable<Timestamp>)]
    pub mission_date: Option<NaiveDateTime>,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub time: Option<String>,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub email: Option<String>,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub phone: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub location: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub rewards: Option<String>,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub updated_at: NaiveDateTime,
}

impl MissionModel {
    pub fn from_entity(entity: MissionEntity, crew_count: i64, chief_display_name: String) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            status: entity.status,
            chief_id: entity.chief_id,
            chief_display_name,
            crew_count,
            mission_date: entity.mission_date,
            time: entity.time,
            email: entity.email,
            phone: entity.phone,
            location: entity.location,
            rewards: entity.rewards,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMissionModel {
    pub name: String,
    pub description: Option<String>,
    pub mission_date: Option<NaiveDateTime>,
    pub time: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub rewards: Option<String>,
}
impl AddMissionModel {
    pub fn to_entity(&self, chief_id: i32) -> AddMissionEntity {
        AddMissionEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            status: MissionStatuses::Open.to_string(),
            chief_id,
            mission_date: self.mission_date,
            time: self.time.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            location: self.location.clone(),
            rewards: self.rewards.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMissionModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub mission_date: Option<NaiveDateTime>,
    pub time: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub rewards: Option<String>,
}
impl EditMissionModel {
    pub fn to_entity(&self, chief_id: i32) -> EditMissionEntity {
        EditMissionEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            status: self.status.clone(),
            chief_id,
            mission_date: self.mission_date,
            time: self.time.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            location: self.location.clone(),
            rewards: self.rewards.clone(),
        }
    }
}
