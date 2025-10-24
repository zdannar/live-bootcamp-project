use crate::utils::auth;
use crate::AuthAPIError;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn verify_token(
    Json(request): Json<TokenVerificationRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Need to use the request.token.  I need to use the validate thing to make sure it is correct.
    let Ok(claims) = auth::validate_token(&request.token).await else {
        return Err(AuthAPIError::InvalidToken);
    };

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenVerificationRequest {
    pub token: String,
}

impl<T> From<T> for TokenVerificationRequest
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self {
            token: value.to_string(),
        }
    }
}
