use axum::{http::StatusCode, response::IntoResponse};

pub async fn verify_token() -> impl IntoResponse {
    let _resp = StatusCode::OK.into_response();
}
