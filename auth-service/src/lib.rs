use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;
pub use app_state::AppState;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
pub use domain::AuthAPIError;
use domain::{BannedTokenStore, UserStore};
use serde::{Deserialize, Serialize};

use crate::domain::{EmailClient, TwoFACodeStore};

pub type UserStoreType<T> = Arc<RwLock<T>>;

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build<
        T: UserStore + 'static,
        B: BannedTokenStore + 'static,
        F: TwoFACodeStore + 'static,
        E: EmailClient + 'static,
    >(
        app_state: AppState<T, B, F, E>,
        address: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/signup", post(routes::signup))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify-token", post(routes::verify_token))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let local_address = listener.local_addr().unwrap().to_string();

        Ok(Self {
            server: axum::serve(listener, router),
            address: local_address,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials(s) => {
                tracing::warn!(msg = s.to_string());
                (StatusCode::BAD_REQUEST, "Invalid Credentials")
            }
            Self::UserDoesNotExists => (StatusCode::NOT_FOUND, "User not found"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthAPIError::IncorrectCredentials => (StatusCode::NOT_FOUND, "User not found"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid Token"),
            AuthAPIError::TokenStoreError(e) => {
                tracing::error!(msg = e.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    // Create a new PostgreSQL connection pool
    PgPoolOptions::new().max_connections(5).connect(url).await
}
