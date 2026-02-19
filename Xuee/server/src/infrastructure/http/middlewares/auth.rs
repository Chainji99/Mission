use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{StatusCode, header},
};
use crate::config::config_loader::get_jwt_env;
use crate::infrastructure::jwt::{verify_token, jwt_model::Claims};

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header_value| header_value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = if auth_header.starts_with("Bearer ") {
        &auth_header[7..]
    } else {
        auth_header
    };

    let jwt_env = get_jwt_env();
    let claims: Claims = verify_token(token, &jwt_env.secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = claims.sub.parse::<i32>().map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
