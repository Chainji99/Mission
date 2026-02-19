use crate::domain::entities::brawlers::RegisterBrawlerEntity;
use serde::{Deserialize, Serialize};
use diesel::{QueryableByName, sql_types::{Int4, Varchar}};

#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
pub struct BrawlerModel {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Varchar)]
    pub username: String,
    #[diesel(sql_type = Varchar)]
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterBrawlerModel {
    pub username: String,
    pub password: String,
    pub display_name: String,
}

impl RegisterBrawlerModel {
    pub fn to_entity(&self) -> RegisterBrawlerEntity {
        RegisterBrawlerEntity {
            username: self.username.clone(),
            password: self.password.clone(),
            display_name: self.display_name.clone(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadAvatarModel {
    pub display_name: String,
    pub avatar_url: String,
    pub mission_success_count: String,
    pub mission_failure_count: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarUploadRequest {
    #[serde(rename = "base64_string")]
    pub base64_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarUploadResponse {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDisplayNameRequest {
    pub display_name: String,
}
