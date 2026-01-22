use axmu::{http::StatusCode, response::IntoResponse};

pub fn default_router() -> impl IntoResponse {
    (StatusCode::OK, "All right, i'm good").into_response()
}

pub async fn make_error(Path(code: u16)): Path<u16> -> impl IntoResponse {
    let code = 401;
    let status_code = StatusCode::from_u16(code).unwrap_or ;
}