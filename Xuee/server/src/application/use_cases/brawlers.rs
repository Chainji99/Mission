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

async fn upload_base64img(
    &self,
    user_id: i32,
    base64img: Base64Img,
    opt: UploadImageOptions,
) -> Result<UploadedImg> {
    let uploaded_img: UploadedImg = cloudinary::upload(base64img, opt).await?;

    let mut conn: PooledConnection<ConnectionManager<...>> = Arc::clone(&self.db_pool).get()?;

    diesel::update(brawlers::table)
        .filter(brawlers::id.eq(user_id))
        .set((
            brawlers::avatar_url.eq(uploaded_img.url.clone()),
            brawlers::avatar_public_id.eq(uploaded_img.public_id.clone()),
        ))
        .execute(&mut conn)?;

    Ok(uploaded_img)
}