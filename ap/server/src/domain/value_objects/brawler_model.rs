use crate::domain::entities::brawlers::RegisterBrawlerEntity;

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
    #[serde(rename = "VarChar")]
    pub display_name: String,
    #[serde(rename = "VarChar")]
    pub avatar_url: String,
    #[serde(rename = "Integer")]
    pub mission_success_count: String,
    #[serde(rename = "Integer")]
    pub mission_failure_count: String,
}