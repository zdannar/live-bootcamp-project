use crate::UserStore;
use crate::{
    domain::{AuthAPIError, User},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn signup<T: UserStore>(
    State(state): State<AppState<T>>,
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
            );
        }
    };

    let mut user_store = state.user_store.write().await;

    let (code, response_msg) = match user_store.add_user(user).await {
        Ok(_) => (
            StatusCode::CREATED,
            SignupResponse {
                message: "User created successfully!".to_string(),
            },
        ),
        Err(e) => {
            let x = e.into_response();

            (
                StatusCode::CONFLICT,
                SignupResponse {
                    message: "User already exists".to_string(),
                },
            )
        }
    };
    (code, Json(response_msg))
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
