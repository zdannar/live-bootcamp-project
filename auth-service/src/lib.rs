use axum::{serve::Serve, Router};
use std::error::Error;
use tower_http::services::ServeDir;

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new().nest_service("/", ServeDir::new("assets"));

        Ok(Self {
            server: axum::serve(tokio::net::TcpListener::bind(address).await?, router),
            address: address.to_owned(),
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
