use crate::helpers::{assert_success_and_context_type, TestApp};
use auth_service::utils::constants::JWT_COOKIE_NAME;
use reqwest::Url;

// #[tokio::test]
// async fn logout_returns_ok() {
//     let app = TestApp::new().await;
//     let response = app.post_logout().await;
//     // TODO: I don't like this... I want to use enum from reqwest.
//     assert_eq!(response.status().as_u16(), 200);
// }

// use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    let response = app.post_logout().await;
    assert_success_and_context_type(&response, 400, None);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

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
