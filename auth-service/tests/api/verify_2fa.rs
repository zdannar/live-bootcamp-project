use crate::helpers::{assert_success_and_context_type, TestApp};
use crate::requests;

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
    assert_success_and_context_type(&response, 200, None);
}
