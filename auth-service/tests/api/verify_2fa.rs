use auth_service::domain::{Email, TwoFACodeStore};
use auth_service::routes::TwoFactorAuthResponse;
use auth_service::utils::constants::JWT_COOKIE_NAME;

use crate::helpers::{assert_success_and_context_type, get_random_email, TestApp};
use crate::requests;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let response = app.post_verify_2fa(&serde_json::json!("MALFORMED!")).await;
    assert_success_and_context_type(&response, 422, None);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let random_email = get_random_email();
    let app = TestApp::new().await;

    let req = requests::Verify2FARequest {
        email: random_email,
        login_attempt_id: "Something",
        two_fa_code: "two_fa_code",
    };

    let response = app.post_verify_2fa(&req).await;
    assert_success_and_context_type(&response, 400, None);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // TODO: This should be a helper function to:
    // - sign up
    // - Login

    let app = TestApp::new().await;
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

    let two_fa_response = response.json::<TwoFactorAuthResponse>().await.unwrap();

    let req = requests::Verify2FARequest {
        email: random_email.as_ref(),
        login_attempt_id: two_fa_response.login_attempt_id,
        two_fa_code: "000000123",
    };

    let response = app.post_verify_2fa(&req).await;
    assert_success_and_context_type(&response, 401, None);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail.
    let app = TestApp::new().await;
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

    // First login
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    // Get the old login code.
    let (login_attempt_id, first_two_fa_code) =
        app.two_fa_code_store.get_code(&random_email).await.unwrap();

    // Second login.  Code should have changed in the backend.
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    // Now verify the code
    let req = requests::Verify2FARequest {
        email: random_email.as_ref(),
        login_attempt_id: login_attempt_id.as_ref(),
        two_fa_code: first_two_fa_code.as_ref(),
    };

    let response = app.post_verify_2fa(&req).await;
    assert_success_and_context_type(&response, 401, None);
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    // Make sure to assert the auth cookie gets set

    let app = TestApp::new().await;
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

    // First login
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    // Get the old login code.
    let (login_attempt_id, two_fa_code) =
        app.two_fa_code_store.get_code(&random_email).await.unwrap();

    // Now verify the code
    let req = requests::Verify2FARequest {
        email: random_email.as_ref(),
        login_attempt_id: login_attempt_id.as_ref(),
        two_fa_code: two_fa_code.as_ref(),
    };

    let response = app.post_verify_2fa(&req).await;
    assert_success_and_context_type(&response, 200, None);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
    assert!(!auth_cookie.value().is_empty());
}
