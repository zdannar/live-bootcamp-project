use crate::helpers::{assert_success_and_context_type, get_random_email, TestApp};
use crate::requests;
use crate::requests::VerifyTokenRequest;
use auth_service::domain::{BannedTokenStore, Email};
use auth_service::utils::auth::generate_auth_token;
use sqlx::PgPool;

#[sqlx::test]
async fn should_return_422_if_malformed_input(pool: PgPool) {
    let app = TestApp::new(pool).await;
    let response = app.post_verify_token(&"malformed".to_string()).await;
    assert_success_and_context_type(&response, 422, None);
}

#[sqlx::test]
async fn should_return_200_valid_token(pool: PgPool) {
    let app = TestApp::new(pool).await;
    let email = Email::parse(get_random_email()).unwrap();
    let request = VerifyTokenRequest {
        token: generate_auth_token(&email).unwrap(),
    };
    let response = app.post_verify_token(&request).await;
    assert_success_and_context_type(&response, 200, None);
}

#[sqlx::test]
async fn should_return_401_if_invalid_token(pool: PgPool) {
    let app = TestApp::new(pool).await;
    let response = app
        .post_verify_token(&requests::VerifyTokenRequest { token: "SOMETOKEN" })
        .await;
    assert_success_and_context_type(&response, 401, None);
}

#[sqlx::test]
async fn should_return_401_if_banned_token(pool: PgPool) {
    let app = TestApp::new(pool).await;
    let email = Email::parse(get_random_email()).unwrap();
    let token = generate_auth_token(&email).unwrap();
    app.banned_token_store.store(&token).await.unwrap();

    let request = VerifyTokenRequest {
        token: token.clone(),
    };

    let response = app.post_verify_token(&request).await;
    assert_success_and_context_type(&response, 401, None);
}
