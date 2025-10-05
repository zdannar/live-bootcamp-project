use crate::helpers::TestApp;
use crate::requests;

static TEXT_HTML: &str = "text/html";
static APPLICATION_JSON: &str = "application/json";

fn assert_success_and_context_type(
    response: reqwest::Response,
    status_code: u16,
    content_type: &str,
) {
    assert_eq!(response.status().as_u16(), status_code);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        content_type
    );
}

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;
    let response = app.get_root().await;
    assert_success_and_context_type(response, 200, TEXT_HTML);
}

#[tokio::test]
async fn login_returns_created() {
    let app = TestApp::new().await;
    let response = app
        .post_login(&requests::LoginRequest {
            email: "someone@somewhere",
            password: "password",
        })
        .await;
    assert_success_and_context_type(response, 201, APPLICATION_JSON);
}

#[tokio::test]
async fn logout_returns_ok() {
    let app = TestApp::new().await;
    let response = app.post_logout().await;
    // TODO: I don't like this... I want to use enum from reqwest.
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn signup_returns_created() {
    let app = TestApp::new().await;
    let response = app
        .post_signup(&requests::SignupRequest {
            email: "someone@somewhere",
            password: "password",
            requires_2fa: false,
        })
        .await;
    assert_success_and_context_type(response, 201, APPLICATION_JSON);
}

#[tokio::test]
async fn post_verify_2fa_ok() {
    let app = TestApp::new().await;
    let response = app
        .post_verify_2fa(&requests::Verify2FARequest {
            email: "someone@somewhere",
            login_attempt_id: "asdf",
            two_fa_code: "some 2fa code",
        })
        .await;
    assert_success_and_context_type(response, 201, APPLICATION_JSON);
}

#[tokio::test]
async fn post_verify_token_ok() {
    let app = TestApp::new().await;
    let response = app
        .post_verify_token(&requests::VerifyTokenRequest { token: "SOMETOKEN" })
        .await;
    assert_success_and_context_type(response, 201, APPLICATION_JSON);
}
