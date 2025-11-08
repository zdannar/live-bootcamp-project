use crate::helpers::{assert_success_and_context_type, get_random_email, TestApp};
use auth_service::domain::{BannedTokenStore, Email};
use auth_service::utils::{auth, constants::JWT_COOKIE_NAME};
use reqwest::Url;
use sqlx::PgPool;

#[sqlx::test]
async fn should_return_400_if_jwt_cookie_missing(pool: PgPool) {
    let app = TestApp::new(pool).await;
    let response = app.post_logout().await;
    assert_success_and_context_type(&response, 400, None);
}

#[sqlx::test]

async fn should_return_401_if_invalid_token(pool: PgPool) {
    let app = TestApp::new(pool).await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_success_and_context_type(&response, 401, None);
}

#[sqlx::test]
async fn logout_return_200(pool: PgPool) {
    let app = TestApp::new(pool).await;

    let valid_token =
        auth::generate_auth_token(&Email::parse(get_random_email()).unwrap()).unwrap();

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!("{JWT_COOKIE_NAME}={valid_token}; HttpOnly; SameSite=Lax; Secure; Path=/",),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_success_and_context_type(&response, 200, None);

    let is_banned: bool = app
        .banned_token_store
        .exists(&valid_token)
        .await
        .unwrap()
        .into();
    assert!(is_banned, "Token does not exist in banned token store");
}
