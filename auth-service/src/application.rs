use crate::{
    routes,
    utils::tracing::{make_span_with_request_id, on_request, on_response},
};
use std::error::Error;
use tower_http::{cors, services::ServeDir, trace::TraceLayer};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{BannedTokenStore, EmailClient, TwoFACodeStore, UserStore},
    AppState, AuthAPIError,
};

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
            .with_state(app_state)
            .layer(cors::CorsLayer::default())
            .layer(
                // New!
                // Add a TraceLayer for HTTP requests to enable detailed tracing
                // This layer will create spans for each request using the make_span_with_request_id function,
                // and log events at the start and end of each request using on_request and on_response functions.
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = tokio::net::TcpListener::bind(address).await?;
        let local_address = listener.local_addr().unwrap().to_string();

        Ok(Self {
            server: axum::serve(listener, router),
            address: local_address,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
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
