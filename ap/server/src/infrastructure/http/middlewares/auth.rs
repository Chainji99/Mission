use axum::{
    middleware::Next,
    response::Response,
    http::Request,
};

pub async fn auth(req: Request<axum::body::Body>, next: Next) -> Response {
    next.run(req).await
}