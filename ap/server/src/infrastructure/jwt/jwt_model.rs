use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::{Duration, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Passport {
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtEnv {
    pub secret: String,
}

impl Passport {
    pub fn new(user_id: i32, exp: usize) -> Result<Self> {
        let jwt_env_secret = std::env::var("JWT_USER_SECRET")
            .unwrap_or_else(|_| "supersecretkey".to_string());
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (Utc::now() + Duration::seconds(exp as i64)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };
        
        let token = super::generate_token(jwt_env_secret, &claims)?;
        Ok(Passport {
            access_token: token,
        })
    }
}