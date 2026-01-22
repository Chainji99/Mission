use std ::syncs::Arc;

use axum::router;

use crate::{
    application::use_cases::authentication::AuthenticationUseCase,
    infrastructure::Database::{
        postgressal::PostgresBrawlerRepository, request_handlers::authentication_handler,
    },
};


pub fn router(db_pool: Arc<Pool<Postgres>>) -> Router {
    let brawler_repository = Arc::new(PostgresBrawlerRepository::new(db_pool.clone()));
    let authentication_use_case = Arc::new(AuthenticationUseCase::new(brawler_repository));

    Router::new().with_stats(Arc::clone(&authentication_use_case))
        .route("/login", post(authentication_handler::login::<AuthenticationUseCase<PostgresBrawlerRepository>>))
        .with_stats(Arc::clone(&authentication_use_case))
        .route("/register", post(register::<AuthenticationUseCase<PostgresBrawlerRepository>>))  

}

pub async fn register<T>(
    state(user_case: Arc<T>): State<Arc<BrawlersUseCase<T>>>,
    Json(payload): Json<RegisterBrawlerModel>,
) -> impl IntoReponse
where
    T: BrawlerRepository + Send + Sync,
{
    match user_case.register(payload).await {
        Ok(user_id) => (StatusCode::CREATED, user_id.to_string()).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    
    }
}

pub async fn upload_avatar<T>(
    State(brawler_use_case: Arc<T>): State<Arc<BrawlersUseCase<T>>>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UploadAvatarModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawler_use_case
        .upload_base64img(user_id, payload.base64_image).await{
        
        Ok(uploaded_img) => (StatusCode::OK, Json(uploaded_img)).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
