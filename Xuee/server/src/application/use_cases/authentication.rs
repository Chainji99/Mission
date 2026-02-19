use std::sync::Arc;
use anyhow::{Result, anyhow};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use rand::{distr::Alphanumeric, Rng};

use crate::domain::repositories::brawlers::BrawlerRepository;
use crate::domain::entities::brawlers::RegisterBrawlerEntity;
use crate::infrastructure::argon2::{verify, hash};
use crate::infrastructure::services::email_service::EmailService;
use crate::infrastructure::services::google_auth_service::GoogleAuthService;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

#[derive(Deserialize)]
pub struct LoginModel {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Passport {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub display_name: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

pub struct AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
    email_service: Arc<EmailService>,
    google_auth_service: Arc<GoogleAuthService>,
}

impl<T> AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(
        brawler_repository: Arc<T>,
        email_service: Arc<EmailService>,
        google_auth_service: Arc<GoogleAuthService>,
    ) -> Self {
        Self {
            brawler_repository,
            email_service,
            google_auth_service,
        }
    }

    pub async fn login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret = std::env::var("JWT_USER_SECRET").unwrap_or_else(|_| "secret".to_string());
        
        println!("Login attempt for user: {}", login_model.username);
        
        // Find user
        let user = match self.brawler_repository.find_by_username(login_model.username.clone()).await {
            Ok(u) => u,
            Err(e) => {
                println!("User not found or DB Error: {} - Error: {}", login_model.username, e);
                
                // Fallback for development if DB is down
                if std::env::var("STAGE").unwrap_or_default() == "local" && login_model.username == "Chain93" {
                    println!("Applying Mock Login for development user: Chain93");
                    crate::domain::entities::brawlers::BrawlerEntity {
                        id: 93,
                        username: "Chain93".to_string(),
                        password: "".to_string(), // Not needed for mock
                        display_name: "Chain93 (Mock)".to_string(),
                        avatar_url: None,
                        avatar_public_id: None,
                        created_at: chrono::Utc::now().naive_utc(),
                        updated_at: chrono::Utc::now().naive_utc(),
                    }
                } else {
                    return Err(anyhow!("Invalid Username or Database Timeout"));
                }
            }
        };
        
        // Verify password (skip if mock)
        if !user.password.is_empty() && !verify(login_model.password, user.password.clone())? {
            println!("Invalid password for user: {}", login_model.username);
            return Err(anyhow!("Invalid Password"));
        }
        
        println!("Login successful for user: {}", login_model.username);
        
        // Generate Token
        // Increase expiration to 30 days for comfort login
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(30))
            .expect("valid timestamp")
            .timestamp();
            
        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;
        
        Ok(Passport {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: 2592000, // 30 days
            display_name: user.display_name,
            username: user.username,
            avatar_url: user.avatar_url,
            created_at: user.created_at,
        })
    }

    pub fn get_google_auth_url(&self) -> (String, String) {
        self.google_auth_service.get_authorization_url()
    }

    pub async fn login_with_google(&self, code: String) -> Result<Passport> {
        let user_info = self.google_auth_service.verify_code(code).await?;
        let email = user_info.email.clone();

        // Check if user exists
        let user_result = self.brawler_repository.find_by_username(email.clone()).await;

        let user_id = match user_result {
            Ok(user) => user.id,
            Err(_) => {
                // Register new user
                let password: String = rand::rng()
                    .sample_iter(&Alphanumeric)
                    .take(16)
                    .map(char::from)
                    .collect();
                
                let hashed_password = hash(password)?;

                let register_entity = RegisterBrawlerEntity {
                    username: email.clone(),
                    password: hashed_password,
                    display_name: user_info.name.clone(),
                };

                let id = self.brawler_repository.register(register_entity).await?;
                
                // Send welcome email
                let _ = self.email_service.send_welcome_email(&email, &user_info.name).await;
                
                id
            }
        };

        // Fetch user to get all fields for Passport
        let user = self.brawler_repository.find_by_username(email).await?;

        // Generate Token
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        // Increase expiration to 30 days for comfort login
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(30))
            .expect("valid timestamp")
            .timestamp();
            
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;
        
        Ok(Passport {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: 2592000,
            display_name: user.display_name,
            username: user.username,
            avatar_url: user.avatar_url,
            created_at: user.created_at,
        })
    }

    pub async fn request_password_reset(&self, username: String) -> Result<()> {
        let user = self.brawler_repository.find_by_username(username.clone()).await?;
        
        // Generate a short-lived token (1 hour)
        let secret = std::env::var("JWT_USER_SECRET").unwrap_or_else(|_| "secret".to_string());
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("valid timestamp")
            .timestamp();
            
        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        // Send Email
        self.email_service.send_password_reset_email(&user.username, &user.display_name, &token).await?;

        Ok(())
    }

    pub async fn reset_password(&self, token: String, new_password: String) -> Result<()> {
        let secret = std::env::var("JWT_USER_SECRET").unwrap_or_else(|_| "secret".to_string());
        
        // Decode and validate token
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;

        let user_id: i32 = token_data.claims.sub.parse()?;
        
        // Hash new password
        let hashed_password = hash(new_password)?;
        
        // Update password in DB
        self.brawler_repository.update_password(user_id, hashed_password).await?;

        Ok(())
    }
}
