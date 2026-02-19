use axum::http::StatusCode;
use axum::response::IntoResponse;

pub fn default_router() -> impl IntoResponse {
    (StatusCode::OK, "All right, i'm good").into_response()
}

pub async fn make_error() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Error").into_response()
}