// use crate::domain::{Email, Login, Password};
// use crate::utils::auth;
// use crate::UserStore;
// use crate::{
//     domain::{AuthAPIError, User},
//     AppState,
// };
// use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
// use axum_extra::extract::{cookie, CookieJar};
// use serde::{Deserialize, Serialize};

// pub async fn logout<T: UserStore>(
//     State(state): State<AppState<T>>,
//     jar: CookieJar,
//     // Json(request): Json<LogRequest>,
// ) -> impl IntoResponse {
//     jar.

//     StatusCode::OK.into_response()
// }

use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let Some(cookie) = jar.get(JWT_COOKIE_NAME) else {
        return (jar, Err(AuthAPIError::MissingToken));
    };

    let token = cookie.value().to_owned();
    // TODO: Fix
    // let claims = validate_token(&token).await.unwrap();

    let Ok(claims) = validate_token(&token).await else {
        return (jar, Err(AuthAPIError::InvalidToken));
    };

    (jar, Ok(StatusCode::OK))
}

// pub struct LogoutResponse {
//     error: String,
// }
