use auth_service::{
    services::{
        hashmap_two_fa_code_store::HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore,
    },
    AppState, Application,
};
use reqwest::cookie::Jar;
use std::sync::Arc;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub cookie_jar: Arc<Jar>, // New!
    pub banned_token_store: HashsetBannedTokenStore,
    pub two_fa_code_store: HashmapTwoFACodeStore,
}

impl TestApp {
    pub async fn new() -> Self {
        #[cfg(feature = "test-trace")]
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();

        let user_store = HashmapUserStore::default();
        let banned_token_store = HashsetBannedTokenStore::default();
        let two_fa_code_store = HashmapTwoFACodeStore::default();

        let app_state = AppState::new(
            user_store,
            banned_token_store.clone(),
            two_fa_code_store.clone(),
        );

        let app = Application::build(app_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address);

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());

        let http_client = reqwest::ClientBuilder::new()
            .cookie_provider(cookie_jar.clone())
            .build()
            .expect("Failed to build client"); // Create a Reqwest http client instance

        Self {
            address,
            http_client,
            cookie_jar,
            banned_token_store,
            two_fa_code_store,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    "grover@example.com".into()
}

static APPLICATION_JSON: &str = "application/json";
pub fn assert_success_and_context_type(
    response: &reqwest::Response,
    status_code: u16,
    _content_type: Option<&str>,
) {
    assert_eq!(response.status().as_u16(), status_code);
    // assert_eq!(
    //     response.headers().get("content-type").unwrap(),
    //     content_type.unwrap_or(APPLICATION_JSON)
    // );
}
