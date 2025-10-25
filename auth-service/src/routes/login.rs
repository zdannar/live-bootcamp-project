use crate::domain::{BannedTokenStore, Email, Login, Password};
use crate::utils::auth;
use crate::UserStore;
use crate::{
    domain::{AuthAPIError, User},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::{cookie, CookieJar};
use serde::{Deserialize, Serialize};

pub async fn login<T: UserStore, B: BannedTokenStore>(
    State(state): State<AppState<T, B>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // OK.  I need to valate the credentails.
    let login = match Login::try_from(request) {
        Err(e) => return (jar, Err(e.into())),
        Ok(l) => l,
    };

    // We are going to have to validate password and set cookies and stuff.
    let user_store = &state.user_store.read().await;
    let user = match user_store.get_user(&login.email).await {
        Err(e) => return (jar, Err(e.into())),
        Ok(u) => u,
    };

    let Ok(auth_cookie) = auth::generate_auth_cookie(&user.email) else {
        return (jar, Err(AuthAPIError::UnexpectedError));
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
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
