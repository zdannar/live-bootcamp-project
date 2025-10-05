use crate::helpers::TestApp;

#[tokio::test]
async fn logout_returns_ok() {
    let app = TestApp::new().await;
    let response = app.post_logout().await;
    // TODO: I don't like this... I want to use enum from reqwest.
    assert_eq!(response.status().as_u16(), 200);
}
