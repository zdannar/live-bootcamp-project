use redis::Commands;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    domain::{BannedTokenDetails, IsBannedToken},
    utils::auth::TOKEN_TTL_SECONDS,
    BannedTokenStore, BannedTokenStoreError,
};

#[derive(Clone)]
pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<redis::Client>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: redis::Client) -> Self {
        Self {
            conn: Arc::new(RwLock::new(conn)),
        }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn store(&self, token: &str) -> Result<(), BannedTokenStoreError> {
        let mut redis_client = self.conn.write().await;
        redis_client
            .set_ex(get_key(token), token, TOKEN_TTL_SECONDS as u64)
            .map_err(|e| BannedTokenStoreError::StoreError(e.to_string()))
    }

    async fn exists(&self, token: &str) -> Result<IsBannedToken, BannedTokenStoreError> {
        let mut redis_client = self.conn.write().await;
        let value: Option<String> = redis_client
            .get(get_key(token))
            .map_err(|e| BannedTokenStoreError::StoreError(e.to_string()))?;

        Ok(match value {
            Some(_) => IsBannedToken::Banned(BannedTokenDetails::default()),
            None => IsBannedToken::NotBanned,
        })
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
