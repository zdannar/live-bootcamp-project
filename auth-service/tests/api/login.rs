use crate::helpers::{assert_success_and_context_type, TestApp};
use crate::requests;

#[tokio::test]
async fn login_returns_created() {
    let app = TestApp::new().await;
    let response = app
        .post_login(&requests::LoginRequest {
            email: "someone@somewhere",
            password: "password",
        })
        .await;
    assert_success_and_context_type(&response, 201, None);
}
