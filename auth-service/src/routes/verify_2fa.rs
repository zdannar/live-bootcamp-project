use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    domain::{
        BannedTokenStore, Email, EmailClient, LoginAttemptId, TwoFACode, TwoFACodeStore, UserStore,
    },
    utils::auth,
    AppState, AuthAPIError,
};

pub async fn verify_2fa<T: UserStore, B: BannedTokenStore, F: TwoFACodeStore, E: EmailClient>(
    jar: CookieJar,
    State(state): State<AppState<T, B, F, E>>,
    Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let code_store = state.two_fa_code_store.read().await;

    let (Ok(email), Ok(login_attempt_id), Ok(two_fa_code)) = (
        Email::parse(request.email),
        LoginAttemptId::parse(request.login_attempt_id),
        TwoFACode::parse(request.two_fa_code),
    ) else {
        return (jar, Ok(StatusCode::BAD_REQUEST));
    };

    let (store_login_attempt_id, store_two_fa_code) = match code_store.get_code(&email).await {
        Ok(v) => v,
        Err(crate::domain::TwoFACodeStoreError::LoginAttemptIdNotFound) => {
            return (jar, Err(AuthAPIError::UserDoesNotExists))
        }
        Err(crate::domain::TwoFACodeStoreError::UnexpectedError) => {
            return (jar, Err(AuthAPIError::UnexpectedError))
        }
    };

    // Now verify that the login attempt lines up and two_fa_code match.
    let status_code =
        if login_attempt_id == store_login_attempt_id && two_fa_code == store_two_fa_code {
            StatusCode::OK
        } else {
            StatusCode::UNAUTHORIZED
        };

    let Ok(auth_cookie) = auth::generate_auth_cookie(&email) else {
        return (jar, Err(AuthAPIError::UnexpectedError));
    };

    (jar.add(auth_cookie), Ok(status_code))
}

#[derive(Clone, Debug, Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}
