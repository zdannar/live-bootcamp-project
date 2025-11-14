use crate::helpers::{assert_success_and_context_type, get_random_email, TestApp};
use crate::requests::SignupRequest;
use auth_service::routes::SignupResponse;
use auth_service::ErrorResponse;
use sqlx::PgPool;

const VALID_PASSWORD: &str = "validpassword";
const INVALID_PASSWORD: &str = "invalid";
const VALID_EMAIL: &str = "chuck@chuck.com";
const INVALID_EMAIL: &str = "chuck.chuck.com";

#[sqlx::test]
async fn should_return_201_if_valid_input(pool: PgPool) {
    let app = TestApp::new(pool).await;
    let response = app
        .post_signup(&SignupRequest {
            email: get_random_email(),
            password: "password".into(),
            requires_2fa: false,
        })
        .await;

    assert_success_and_context_type(&response, 201, None);

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

#[sqlx::test]
async fn should_return_400_if_invalid_input(pool: PgPool) {
    let invalid_signup_requests = [
        SignupRequest::new(INVALID_EMAIL, VALID_PASSWORD, false),
        SignupRequest::new(VALID_EMAIL, INVALID_PASSWORD, false),
    ];

    let app = TestApp::new(pool).await;

    for signup_request in invalid_signup_requests.iter() {
        let response = app.post_signup(signup_request).await;
        assert_success_and_context_type(&response, 400, None);
    }
}

#[sqlx::test]

async fn should_return_409_if_email_already_exists(pool: PgPool) {
    let app = TestApp::new(pool).await;
    // Call the signup route twice. The second request should fail with a 409 HTTP status code
    let signup_request = SignupRequest {
        email: get_random_email(),
        password: VALID_PASSWORD.to_string(),
        requires_2fa: false,
    };

    let mut response = app.post_signup(&signup_request).await;
    // assert_success_and_context_type(&response, 201, None);
    assert_eq!(
        response.status().as_u16(),
        201,
        "Email should not exist in database"
    );

    response = app.post_signup(&signup_request).await;
    assert_eq!(
        response.status().as_u16(),
        409,
        "Email should already exist in database"
    );

    response = app.post_signup(&signup_request).await;
    let round_two = response.json::<serde_json::Value>().await;
    println!("Debug Round two>> {round_two:?}");

    response = app.post_signup(&signup_request).await;

    assert_eq!(
        response
            // .json::<ErrorResponse>()
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
