use crate::helpers::{assert_success_and_context_type, TestApp};
static TEXT_HTML: &str = "text/html";

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;
    let response = app.get_root().await;
    assert_success_and_context_type(&response, 200, Some(TEXT_HTML));
}
