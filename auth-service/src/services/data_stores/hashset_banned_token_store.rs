use crate::domain::{BannedTokenStore, BannedTokenStoreError, IsBannedToken};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct HashsetBannedTokenStore {
    hashset: Arc<RwLock<HashSet<String>>>,
}

impl Default for HashsetBannedTokenStore {
    fn default() -> Self {
        Self {
            hashset: Arc::new(RwLock::new(HashSet::new())),
        }
    }
}

impl Clone for HashsetBannedTokenStore {
    fn clone(&self) -> Self {
        Self {
            hashset: self.hashset.clone(),
        }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn store(&self, token: &str) -> Result<(), BannedTokenStoreError> {
        let mut set = self.hashset.write().await;
        set.insert(token.to_owned());
        Ok(())
    }

    async fn exists(&self, token: &str) -> Result<IsBannedToken, BannedTokenStoreError> {
        let set = self.hashset.read().await;
        match set.get(token) {
            Some(_v) => Ok(IsBannedToken::Banned(
                crate::domain::BannedTokenDetails::default(),
            )),
            None => Ok(IsBannedToken::NotBanned),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST_TOKEN: &str = "testtoken";

    #[tokio::test]
    async fn store_token_and_does_exists() {
        let hstore = HashsetBannedTokenStore::default();
        hstore.store(TEST_TOKEN).await.unwrap();
        let is_banned: bool = hstore.exists(TEST_TOKEN).await.unwrap().into();
        assert!(is_banned);
    }

    #[tokio::test]
    async fn token_should_not_exist() {
        let hstore = HashsetBannedTokenStore::default();
        let is_banned = hstore.exists(TEST_TOKEN).await.unwrap();
        assert_eq!(is_banned, IsBannedToken::NotBanned);
    }
}
