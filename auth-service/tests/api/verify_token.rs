use crate::helpers::{assert_success_and_context_type, TestApp};
use crate::requests;

#[tokio::test]
async fn post_verify_token_ok() {
    let app = TestApp::new().await;
    let response = app
        .post_verify_token(&requests::VerifyTokenRequest { token: "SOMETOKEN" })
        .await;
    assert_success_and_context_type(&response, 200, None);
}
