use axum::{http::StatusCode, response::IntoResponse};

pub async fn verify_token() -> impl IntoResponse {
    let resp = StatusCode::OK.into_response();
}
