use anyhow::Result;
use diesel_async::{
    pooled_connection::bb8::Pool,
    pooled_connection::AsyncDieselConnectionManager,
    AsyncPgConnection,
};

pub type PgPoolSquad = Pool<AsyncPgConnection>;

pub async fn establish_connection(database_url: &str) -> Result<PgPoolSquad> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder().build(config).await?;
    Ok(pool)
}