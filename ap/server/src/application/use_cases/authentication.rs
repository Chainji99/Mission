use std::sync::Arc;

use anyhow::Result;
use argon2::{password_hash::SaltString, Argon2};
use chrono::Duration;
use rand::thread_rng;

use crate::{
    domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        repositories::brawlers::BrawlerRepository,
        value_objects::brawler_model::{LoginModel, RegisterBrawlerModel},
    },
    infrastructure::jwt::{
        jwt_model::{Claims, Passport},
        generate_token,
    },
};

pub struct AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}

impl<T> AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }

    pub async fn login(&self, login_model: LoginModel) -> Result<Passport> {
        let username = login_model.username.clone();

        let user = self.brawler_repository.find_by_username(username).await?;
        let hashed_password = user.password;

        // Verify password
        use argon2::Argon2;
        use argon2::password_hash::PasswordHash;

        let parsed_hash = PasswordHash::new(&hashed_password)?;
        let argon2 = Argon2::default();

        if argon2
            .verify_password(login_model.password.as_bytes(), &parsed_hash)
            .is_err()
        {
            return Err(anyhow::anyhow!("Invalid Password"));
        }

        let secret = std::env::var("JWT_USER_SECRET")
            .unwrap_or_else(|_| "supersecretkey".to_string());
        let ttl: i64 = std::env::var("JWT_TTL")
            .unwrap_or_else(|_| "86400".to_string())
            .parse()
            .unwrap_or(86400);

        let claims = Claims {
            sub: user.id.to_string(),
            exp: (chrono::Utc::now() + Duration::seconds(ttl)).timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };

        let token = generate_token(secret, &claims)?;

        Ok(Passport {
            access_token: token,
        })
    }

    pub async fn register(&self, register_model: RegisterBrawlerModel) -> Result<i32> {
        // Hash password with Argon2
        let salt = SaltString::generate(&mut thread_rng());
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(register_model.password.as_bytes(), &salt)?
            .to_string();

        let new_user = RegisterBrawlerEntity {
            username: register_model.username,
            password: password_hash,
            display_name: register_model.display_name,
        };

        let user_id = self.brawler_repository.register(new_user).await?;
        Ok(user_id)
    }
}