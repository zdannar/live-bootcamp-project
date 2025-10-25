use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    domain::{AuthAPIError, BannedTokenStore, UserStore},
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
    AppState,
};

pub async fn logout<T: UserStore, B: BannedTokenStore>(
    jar: CookieJar,
    State(state): State<AppState<T, B>>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let Some(cookie) = jar.get(JWT_COOKIE_NAME) else {
        return (jar, Err(AuthAPIError::MissingToken));
    };

    // TODO: Fix unwrap

    let token = cookie.value().to_owned();

    let Ok(_claims) = validate_token(&token).await else {
        return (jar, Err(AuthAPIError::InvalidToken));
    };

    state.banned_token_store.store(&token).await.unwrap();

    (jar, Ok(StatusCode::OK))
}
