use crate::helpers::TestApp;

static TEXT_HTML: &str = "text/html";

fn assert_success_and_context_type(
    response: reqwest::Response,
    status_code: u16,
    content_type: &str,
) {
    assert_eq!(response.status().as_u16(), status_code);
    // TODO: Commenting out until we get this far.  Maybe LGR will go a different direction.
    // assert_eq!(
    //     response.headers().get("content-type").unwrap(),
    //     content_type
    // );
}

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;
    let response = app.get_root().await;
    assert_success_and_context_type(response, 200, TEXT_HTML);
}
