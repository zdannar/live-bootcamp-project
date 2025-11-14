use crate::domain::{BannedTokenStore, EmailClient, TwoFACodeStore};
use crate::UserStore;
use crate::{
    domain::{AuthAPIError, User},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use secrecy::{ExposeSecret, SecretString, SerializableSecret};
use serde::{Deserialize, Serialize};

#[tracing::instrument(name = "Signup", skip_all)]
pub async fn signup<T: UserStore, B: BannedTokenStore, F: TwoFACodeStore, E: EmailClient>(
    State(state): State<AppState<T, B, F, E>>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let user = match User::try_from(request) {
        Ok(u) => u,
        Err(e) => {
            return Ok((
                StatusCode::BAD_REQUEST,
                Json(SignupResponse {
                    message: e.to_string(),
                }),
            )
                .into_response());
        }
    };

    let mut user_store = state.user_store.write().await;

    match user_store.add_user(user).await {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(SignupResponse {
                message: "User created successfully!".to_string(),
            }),
        )
            .into_response()),
        Err(e) => Err(AuthAPIError::from(e)),
    }
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: SecretString,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl SignupRequest {
    pub fn new<T: Into<String>, U: Into<String>>(
        email: T,
        password: U,
        requires_2fa: bool,
    ) -> Self {
        Self {
            email: email.into(),
            password: SecretString::from(password.into()),
            requires_2fa,
        }
    }
}

impl TryFrom<SignupRequest> for User {
    type Error = AuthAPIError;
    fn try_from(value: SignupRequest) -> Result<Self, Self::Error> {
        // TODO: Need to look at this.
        Ok(User::new(
            value.email,
            // TODO: This seems like a problem.
            value.password.expose_secret().to_string(),
            value.requires_2fa,
        )?)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
