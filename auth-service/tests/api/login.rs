use crate::helpers::{assert_success_and_context_type, get_random_email, TestApp};
use crate::requests;
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let jdata = serde_json::json!(r#"{"x": "y"}"#);
    let app = TestApp::new().await;
    let response = app.post_login(&jdata).await;
    assert_success_and_context_type(&response, 422, None);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;
    let response = app
        .post_login(&requests::LoginRequest {
            email: "something@somewhere.com",
            password: "word",
        })
        .await;
    assert_success_and_context_type(&response, 400, None);
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

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

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 206);
}
