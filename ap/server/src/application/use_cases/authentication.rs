use std::string;

use anyhow::Result;
use argon2::password_hash::{self, rand_core::le};
use chrono::Utc;

use crate::domain::entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity};
pub struct  AuthenticationUseCase<T> 
where T : BrawlerRepository + Send + Sync 
{
    brawler_repository: Arc<T>,
}

pub struct AuthenticationUseCase<T> 
where T : BrawlerRepository + Send + Sync (brawler_repository: Arc<T>,) 
impl<T> AuthenticationUseCase<T> where T : BrawlerRepository + Send + Sync
{
    pub fn new(brawler_repository: Arc<T>)-> Self {
        Self { brawler_repository }
    }

    pub fn login(&self, loginModel: LoginModel)-> Result<Passport> {
        let seceret: String = get_user_sercret();
        let username: String = login_model.username.clone();

        let user = BrawlerEntity = self.brawler_repository.find_by_username(username).await?;
        let hash_password: String = user.password;

        if !argon2::verify(login_model.password, &hashed_password)? {
            return Err(anyhow::anyhow!("Invalid Password !!"))
        }

        let claims: Claims = Claims { 
            sub: user.id.to_string(), 
            exp: (Utc::now() + Duration::day(3)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        }

        let token: String = generate_token(&claims, secret)?;

        ok(Passport{
            access_token: token,
        })
        
    }

}