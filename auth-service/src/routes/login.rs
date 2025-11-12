use crate::domain::{
    BannedTokenStore, Email, EmailClient, Login, LoginAttemptId, TwoFACode, TwoFACodeStore,
};
use crate::utils::auth;
use crate::UserStore;
use crate::{domain::AuthAPIError, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

pub async fn login<T: UserStore, B: BannedTokenStore, F: TwoFACodeStore, E: EmailClient>(
    State(state): State<AppState<T, B, F, E>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // OK.  I need to valate the credentails.
    let login = match Login::try_from(request) {
        Err(e) => return (jar, Err(e)),
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
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

async fn handle_2fa<T: UserStore, B: BannedTokenStore, F: TwoFACodeStore, E: EmailClient>(
    email: &Email,
    state: &AppState<T, B, F, E>,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    // First, we must generate a new random login attempt ID and 2FA code
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    // let ds = state.two_fa_code_store.add_code().await.unwrap();
    let code_store = state.two_fa_code_store.write().await;

    match code_store
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
        .map_err(|e| AuthAPIError::UnexpectedError)
    {
        Ok(_) => (),
        Err(e) => return (jar, Err(e)),
    };

    let Ok(_email_response) = state
        .email_client
        .send_email(email, "Two FA Code Validation", two_fa_code.as_ref())
        .await
    else {
        return (jar, Err(AuthAPIError::UnexpectedError));
    };

    let resp = (
        StatusCode::PARTIAL_CONTENT,
        Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
            message: "2FA required".to_owned(),
            login_attempt_id: login_attempt_id.as_ref().to_owned(),
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
    let Ok(auth_cookie) = auth::generate_auth_cookie(email) else {
        return (jar, Err(AuthAPIError::UnexpectedError));
    };

    (
        jar.add(auth_cookie),
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
