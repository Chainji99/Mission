use axum::{http::StatusCode, response::IntoResponse, extract::Path};

pub async fn default_router() -> impl IntoResponse {
    (StatusCode::OK, "All right, i'm good").into_response()
}

pub async fn make_error(Path(code): Path<u16>) -> impl IntoResponse {
    let status_code = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status_code, "Error generated").into_response()
}
