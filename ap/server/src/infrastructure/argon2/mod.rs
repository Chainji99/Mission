use anyhow::Result;
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{PasswordHash, PasswordHasher, SaltString, rand_core::OsRng}
};

pub fn hash(password: String) -> Result<String> {
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let bytes_password: &[u8] = password.as_bytes();

    let argon2: Argon2<'_> = Argon2::default();

    let value: PasswordHash<'_> = argon2
        .hash_password(bytes_password, &salt)
        .map_err(|e: Error | anyhow::anyhow!(e.to_string()))?;
    Ok(value.to_string())
}

pub fn verify(password: String, hash: String) -> Result<bool> {
    let parsed_hash: PasswordHash<'_> = 
    PasswordHash::new(&hash).map_err(|e: Error | anyhow::anyhow!(e.to_string()))?;

    let bytes_password: &[u8] = password.as_bytes();
    let argon2: Argon2<'_> = Argon2::default()
        .verify_password(bytes_password, & parsed_hash)
        .or_else(|_| Ok(false))?;
    Ok(false)

}