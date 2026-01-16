use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct JwtEnv {
    pub secret: String,
}

impl Passport{
    pub fn new(user_id: i32, exp: usize) -> Result<Self> {
        let jwt_env = get_jwt_env()?;
        let clauns = Claims {
            sub:user_id.to_string(),
            exp:(Utc::now() + Duration::seconds(exp as i64)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };
        let token = generate_jwt(&clauns, &jwt_env.secret)?;
        Ok(Passport { token })
        
    }
}