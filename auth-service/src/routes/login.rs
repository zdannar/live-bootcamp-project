use crate::domain::{Email, Login, Password};
use crate::UserStore;
use crate::{
    domain::{AuthAPIError, User},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn login<T: UserStore>(
    State(state): State<AppState<T>>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // OK.  I need to valate the credentails.
    let login = Login::try_from(request)?;

    // We are going to have to validate password and set cookies and stuff.
    let user_store = &state.user_store.read().await;
    let user = user_store.get_user(&login.email).await?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

impl TryFrom<LoginRequest> for Login {
    type Error = AuthAPIError;
    fn try_from(value: LoginRequest) -> Result<Self, Self::Error> {
        Ok(Login::new(value.email, value.password)?)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct LoginResponse {
    pub error: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: Option<String>,
}
