use std::sync::Arc;
use crate::infrastructure::argon2::hash;
use crate::domain::repositories::brawlers::BrawlerRepository;

pub struct BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}

impl<T> BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }
    pub async fn register(&self, mut register_brawler_model: RegisterBrawlerModel) -> Result<i32> {
        let hashed_password = hash(register_brawler_model.password.clone())?;
        register_brawler_model.password = hashed_password;
        let register_entity = register_brawler_model.to_entity();
        let id = self.brawler_repository.register(register_entity).await?;
        Ok(id)
    }
}
pub trait BrawlerRepository {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32>;
    
    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity>;

pub async fn upload_base64img(
    &self,
    user_id: i32,
    base64string: String,
) -> Result<UploadedImg> {
    let opt: UploaldImageOptions = UploadImageOptions {
        folder: Some("avatars/".to_string()),
        public_id: Some(format!("avatar_{}", user_id)),
        transformation: Some("c_fill,g_face,r_max,w_256,h_256".to_string()),
    };

    let base64img: Base64Img = Base64Img::new(base64string)?;
    
    let uploaded: UploadedImg = self.brawler_repository.upload_base64img(user_id, base64img, opt)
    .await?;

    Ok(uploaded)
}
}
