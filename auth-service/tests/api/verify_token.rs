use crate::helpers::TestApp;
use crate::requests;

static APPLICATION_JSON: &str = "application/json";

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
async fn post_verify_token_ok() {
    let app = TestApp::new().await;
    let response = app
        .post_verify_token(&requests::VerifyTokenRequest { token: "SOMETOKEN" })
        .await;
    assert_success_and_context_type(response, 200, APPLICATION_JSON);
}
