use crate::domain::UserValidationError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AuthAPIError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentails: {0}")]
    InvalidCredentials(String),
    #[error("Unexpected error")]
    UnexpectedError,
}

impl From<UserValidationError> for AuthAPIError {
    fn from(value: UserValidationError) -> Self {
        let reason = match value {
            UserValidationError::InvalidEmail(r) => r,
            UserValidationError::InvalidPassword(r) => r,
        };

        // NOTE: Logging or tracing could go on here.
        Self::InvalidCredentials(reason)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials(_s) => {
                // Logging/Tracing could be used here.
                (StatusCode::BAD_REQUEST, "Invalid credentails")
            }
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}
