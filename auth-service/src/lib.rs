use redis::Client;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;
pub use app_state::AppState;
pub mod application;
pub use application::{Application, ErrorResponse};
pub use domain::AuthAPIError;
use domain::{BannedTokenStore, BannedTokenStoreError, UserStore, UserStoreError};

use crate::{
    domain::{EmailClient, TwoFACodeStore, TwoFACodeStoreError},
    utils::constants::DATABASE_URL,
};

pub type UserStoreType<T> = Arc<RwLock<T>>;

#[tracing::instrument(skip_all)]
pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    // Create a new PostgreSQL connection pool
    PgPoolOptions::new().max_connections(5).connect(url).await
}

#[tracing::instrument(skip_all)]
pub async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database!
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}

pub fn get_redis_client(redis_hostname: String) -> redis::RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}
