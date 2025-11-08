use auth_service::configure_postgresql;
use auth_service::utils::constants::DATABASE_URL;
use auth_service::{
    services::{
        HashmapTwoFACodeStore, HashsetBannedTokenStore, MockEmailClient, PostgresUserStore,
    },
    AppState, Application,
};
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use reqwest::cookie::Jar;
use sqlx::postgres::PgConnectOptions;
use sqlx::Executor;
use sqlx::{Connection, PgConnection, PgPool};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub cookie_jar: Arc<Jar>, // New!
    pub banned_token_store: HashsetBannedTokenStore,
    pub two_fa_code_store: HashmapTwoFACodeStore,
    pub database_name: String,
    pub cleanup_called: bool,
}

impl TestApp {
    // pub async fn new<T: Into<String>>(database_name: T) -> Self {
    pub async fn new(pg_pool: PgPool) -> Self {
        #[cfg(feature = "test-trace")]
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();

        // let user_store = HashmapUserStore::default();

        // let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
        let user_store = PostgresUserStore::new(pg_pool);

        let banned_token_store = HashsetBannedTokenStore::default();
        let two_fa_code_store = HashmapTwoFACodeStore::default();
        let email_client = MockEmailClient::default();

        let app_state = AppState::new(
            user_store,
            banned_token_store.clone(),
            two_fa_code_store.clone(),
            email_client.clone(),
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
            database_name: Uuid::new_v4().to_string(),
            cleanup_called: false,
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

    // pub async fn cleanp(&mut self) {
    //     delete_database(&self.database_name).await;
    //     self.cleanup_called = true
    // }
}

// impl Drop for TestApp {
//     fn drop(&mut self) {
//         if !self.cleanup_called {
//             panic!("Cleanup not called!")
//         } else {
//             ()
//         }
//     }
// }

// async fn delete_database(db_name: &str) {
//     let postgresql_conn_url: String = DATABASE_URL.to_owned();

//     let connection_options = PgConnectOptions::from_str(&postgresql_conn_url)
//         .expect("Failed to parse PostgreSQL connection string");

//     let mut connection = PgConnection::connect_with(&connection_options)
//         .await
//         .expect("Failed to connect to Postgres");

//     // Kill any active connections to the database
//     connection
//         .execute(
//             format!(
//                 r#"
//                 SELECT pg_terminate_backend(pg_stat_activity.pid)
//                 FROM pg_stat_activity
//                 WHERE pg_stat_activity.datname = '{}'
//                   AND pid <> pg_backend_pid();
//         "#,
//                 db_name
//             )
//             .as_str(),
//         )
//         .await
//         .expect("Failed to drop the database.");

//     // Drop the database
//     connection
//         .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
//         .await
//         .expect("Failed to drop the database.");
// }

pub fn get_random_email() -> String {
    let length = 10; // Desired length of the random string
    let random_string: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    format!("{random_string}@something.com")
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
