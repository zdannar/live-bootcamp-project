use crate::domain::{BannedTokenStore, EmailClient, TwoFACodeStore};
use crate::UserStore;
use crate::{
    domain::{AuthAPIError, User},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn signup<T: UserStore, B: BannedTokenStore, F: TwoFACodeStore, E: EmailClient>(
    State(state): State<AppState<T, B, F, E>>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let user = match User::try_from(request) {
        Ok(u) => u,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(SignupResponse {
                    message: e.to_string(),
                }),
            )
                .into_response();
        }
    };

    let mut user_store = state.user_store.write().await;

    match user_store.add_user(user).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(SignupResponse {
                message: "User created successfully!".to_string(),
            }),
        )
            .into_response(),
        Err(e) => AuthAPIError::from(e).into_response(),
    }
}

#[derive(Deserialize, Serialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl SignupRequest {
    pub fn new<T: ToString>(email: T, password: T, requires_2fa: bool) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
            requires_2fa,
        }
    }
}

impl TryFrom<SignupRequest> for User {
    type Error = AuthAPIError;
    fn try_from(value: SignupRequest) -> Result<Self, Self::Error> {
        Ok(User::new(value.email, value.password, value.requires_2fa)?)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
