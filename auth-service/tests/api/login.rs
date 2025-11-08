use crate::helpers::{assert_success_and_context_type, get_random_email, TestApp};
use crate::requests;
use auth_service::domain::{Email, TwoFACodeStore};
use auth_service::routes::TwoFactorAuthResponse;
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use sqlx::PgPool;

#[sqlx::test]
async fn should_return_422_if_malformed_credentials(pool: PgPool) {
    let jdata = serde_json::json!(r#"{"x": "y"}"#);
    let app = TestApp::new(pool).await;
    let response = app.post_login(&jdata).await;
    assert_success_and_context_type(&response, 422, None);
}

#[sqlx::test]
async fn should_return_400_if_invalid_input(pool: PgPool) {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new(pool).await;
    let response = app
        .post_login(&requests::LoginRequest {
            email: "something@somewhere.com",
            password: "word",
        })
        .await;
    assert_success_and_context_type(&response, 400, None);
}

#[sqlx::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled(pool: PgPool) {
    let app = TestApp::new(pool).await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[sqlx::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled(pool: PgPool) {
    let app = TestApp::new(pool).await;
    let random_email = Email::parse(get_random_email()).unwrap();

    let signup_body = serde_json::json!({
        "email": random_email.as_ref(),
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email.as_ref(),
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let two_fa_response = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(
        two_fa_response.message,
        "2FA required".to_owned(),
        "->> 2FA is required failed"
    );

    println!("{:?}", app.two_fa_code_store.get_code(&random_email).await);

    // TODO: assert that `json_body.login_attempt_id` is stored inside `app.two_fa_code_store`
    assert!(
        app.two_fa_code_store.get_code(&random_email).await.is_ok(),
        "->> Retrieve from code store failed"
    );
}
