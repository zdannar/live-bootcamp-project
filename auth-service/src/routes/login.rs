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
        Err(e) => {
            return (jar, Err(e.into()));
        }

        Ok(u) => u,
    };

    match user.requires_2fa {
        true => handle_2fa(jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

async fn handle_2fa(
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let resp = (
        StatusCode::PARTIAL_CONTENT,
        Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
            message: "2FA required".to_owned(),
            login_attempt_id: ("123456").to_owned(),
        })),
    );

    (jar, Ok(resp))
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let Ok(auth_cookie) = auth::generate_auth_cookie(&email) else {
        return (jar, Err(AuthAPIError::UnexpectedError));
    };

    let updated_jar = jar.add(auth_cookie);

    (
        updated_jar,
        Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))),
    )
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

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
