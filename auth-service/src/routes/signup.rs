use crate::{domain::User, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    // Create a new `User` instance using data in the `request`
    let user = User::from(request);

    // TODO: Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.
    let mut user_store = state.user_store.write().await;
    user_store.add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
}

#[derive(Deserialize, Serialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl From<SignupRequest> for User {
    fn from(value: SignupRequest) -> Self {
        User::new(value.email, value.password, value.requires_2fa)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
