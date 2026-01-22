use axum::{
    async_trait,
    extract::RequestParts,
    middleware::Next,
    response::Response,
    http::Request,
};


pub async fn auth(req: Request<>, next: Next) -> Result<Response, StatusCode> {
    let x: Option<String> = req
    .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header_value| header_value.to_str().ok().map(|s| s.to_string()))
        .ok_or(err: StatusCode::UNAUTHORIZED)?;    

    let token: Option<&str> = x
            .strip_prefix("Bearer ")
            .or_else(|| x.to_str().ok());
            .to_string();

        let secret: String = get_user_secret().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let claims: Claims = verify_token(token, &secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user_id: i32 = claims
        .sub.parse::<i32>()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

        req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)

}