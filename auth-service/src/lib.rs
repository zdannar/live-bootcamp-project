use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

pub mod domain;
pub mod routes;
pub mod services;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
pub use domain::AuthAPIError;
use domain::UserStore;
use serde::{Deserialize, Serialize};

pub type UserStoreType<T> = Arc<RwLock<T>>;

#[derive(Clone)]
pub struct AppState<T> {
    pub user_store: UserStoreType<T>,
}

impl<T> AppState<T>
where
    T: UserStore + Clone + Sync + Send + 'static,
{
    pub fn new(user_store: T) -> Self {
        Self {
            user_store: Arc::new(RwLock::new(user_store)),
        }
    }
}

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build<T: UserStore + Send + Sync + Clone + 'static>(
        app_state: AppState<T>,
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
            AuthAPIError::InvalidCredentials(_s) => {
                // Logging/Tracing could be used here.
                // (StatusCode::BAD_REQUEST, s.as_str())
                (StatusCode::BAD_REQUEST, "Invalid Credentials")
            }
            Self::UserDoesNotExists => (StatusCode::NOT_FOUND, "User not found"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthAPIError::IncorrectCredentials => (StatusCode::NOT_FOUND, "User not found"),
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}
