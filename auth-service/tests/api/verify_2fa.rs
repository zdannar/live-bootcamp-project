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

async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail.
    todo!()
}
