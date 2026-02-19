use anyhow::Result;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng}
};

pub fn hash(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
    Ok(password_hash.to_string())
}

pub fn verify(password: String, hash: String) -> Result<bool> {
    let parsed_hash = PasswordHash::new(&hash)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
    let argon2 = Argon2::default();
    
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
