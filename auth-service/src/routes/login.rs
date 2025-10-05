use axum::{http::StatusCode, response::IntoResponse};

pub async fn login() -> impl IntoResponse {
    StatusCode::CREATED.into_response()
}
