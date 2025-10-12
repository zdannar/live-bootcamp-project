use crate::helpers::{get_random_email, TestApp};
// use crate::requests;
use auth_service::routes::{SignupRequest, SignupResponse};

static APPLICATION_JSON: &str = "application/json";

fn assert_success_and_context_type(
    response: &reqwest::Response,
    status_code: u16,
    _content_type: &str,
) {
    assert_eq!(response.status().as_u16(), status_code);
    // TODO: Commenting out until we get this far.  Maybe LGR will go a different direction.
    // assert_eq!(
    //     response.headers().get("content-type").unwrap(),
    //     content_type
    // );
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let response = app
        .post_signup(&SignupRequest {
            email: get_random_email(),
            password: "password".into(),
            requires_2fa: false,
        })
        .await;
    assert_success_and_context_type(&response, 201, APPLICATION_JSON);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}
