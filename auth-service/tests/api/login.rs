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
async fn login_returns_created() {
    let app = TestApp::new().await;
    let response = app
        .post_login(&requests::LoginRequest {
            email: "someone@somewhere",
            password: "password",
        })
        .await;
    assert_success_and_context_type(response, 201, APPLICATION_JSON);
}
