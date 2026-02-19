use std::sync::Arc;
use anyhow::Result;

use crate::domain::repositories::brawlers::BrawlerRepository;
use crate::domain::value_objects::brawler_model::{RegisterBrawlerModel, AvatarUploadResponse};
use crate::infrastructure::argon2::hash;
use crate::infrastructure::services::image_storage::ImageStorageService;
use crate::infrastructure::services::email_service::EmailService;

pub struct BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
    email_service: Arc<EmailService>,
}

impl<T> BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>, email_service: Arc<EmailService>) -> Self {
        Self { brawler_repository, email_service }
    }

    pub async fn register(&self, mut register_brawler_model: RegisterBrawlerModel) -> Result<i32> {
        let email_recipient = register_brawler_model.username.clone();
        
        let hashed_password = hash(register_brawler_model.password.clone())?;
        register_brawler_model.password = hashed_password;
        
        let register_entity = register_brawler_model.to_entity();
        let id = self.brawler_repository.register(register_entity).await?;
        
        // Send welcome email
        // We attempt to send to 'username' assuming it is an email.
        let _ = self.email_service.send_welcome_email(&email_recipient, &email_recipient).await;

        Ok(id)
    }

    pub async fn upload_avatar(&self, user_id: i32, base64_string: String) -> Result<AvatarUploadResponse> {
        let url = ImageStorageService::upload(&base64_string).await?;
        self.brawler_repository.update_avatar(user_id, url.clone()).await?;
        Ok(AvatarUploadResponse { url })
    }

    pub async fn update_display_name(&self, user_id: i32, display_name: String) -> Result<()> {
        self.brawler_repository.update_display_name(user_id, display_name).await
    }
}
