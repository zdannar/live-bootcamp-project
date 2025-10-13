use crate::helpers::{get_random_email, TestApp};
use auth_service::routes::{SignupRequest, SignupResponse};
use auth_service::ErrorResponse;

const APPLICATION_JSON: &str = "application/json";
const VALID_PASSWORD: &str = "validpassword";
const INVALID_PASSWORD: &str = "invalid";
const VALID_EMAIL: &str = "chuck@chuck.com";
const INVALID_EMAIL: &str = "chuck.chuck.com";

fn assert_success_and_context_type(
    response: &reqwest::Response,
    status_code: u16,
    _content_type: &str,
) {
    assert_eq!(response.status().as_u16(), status_code);
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

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let invalid_signup_requests = [
        SignupRequest::new(INVALID_EMAIL, VALID_PASSWORD, false),
        SignupRequest::new(VALID_EMAIL, INVALID_PASSWORD, false),
    ];

    let app = TestApp::new().await;

    for signup_request in invalid_signup_requests.iter() {
        let response = app.post_signup(signup_request).await;
        assert_success_and_context_type(&response, 400, APPLICATION_JSON);
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;
    // Call the signup route twice. The second request should fail with a 409 HTTP status code
    let signup_request = SignupRequest::new(VALID_EMAIL, VALID_PASSWORD, false);

    let mut response = app.post_signup(&signup_request).await;
    println!(">> DEBUG: {response:?}");
    assert_success_and_context_type(&response, 201, APPLICATION_JSON);

    response = app.post_signup(&signup_request).await;
    assert_eq!(response.status().as_u16(), 409);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
