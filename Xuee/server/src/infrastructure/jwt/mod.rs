pub mod authentication_model;
pub mod jwt_model;
use anyhow::Result;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub fn generate_token(secret: String, claims: &jwt_model::Claims) -> Result<String> {
    // HSA256
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn generate_token(secret: String, claims: &jwt_model::Claims) -> Result<String> {
    let token = encode(
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token)
}