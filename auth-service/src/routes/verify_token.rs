use crate::domain::{BannedTokenStore, TwoFACodeStore};
use crate::utils::auth;
use crate::UserStore;
use crate::{domain::AuthAPIError, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn verify_token<T: UserStore, B: BannedTokenStore, F: TwoFACodeStore>(
    State(state): State<AppState<T, B, F>>,
    Json(request): Json<TokenVerificationRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let Ok(_claims) = auth::validate_token(&request.token, &*state.banned_token_store).await else {
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
