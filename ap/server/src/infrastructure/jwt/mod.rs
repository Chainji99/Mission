pub mod authentication_model;
pub mod jwt_model;

use anyhow::Result;
use jsonwebtoken::{encode, EncodingKey, Header};
use jwt_model::Claims;

pub fn generate_token(secret: String, claims: &Claims) -> Result<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}