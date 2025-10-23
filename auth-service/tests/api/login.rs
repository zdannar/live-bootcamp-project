use crate::helpers::{assert_success_and_context_type, TestApp};
use crate::requests;

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
