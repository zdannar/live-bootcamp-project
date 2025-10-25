use crate::domain;
use crate::UserStoreType;
use domain::BannedTokenStore;
use domain::UserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState<T, B> {
    pub user_store: UserStoreType<T>,
    pub banned_token_store: Arc<B>,
}

impl<T, B> AppState<T, B>
where
    T: UserStore + Clone + Sync + Send + 'static,
    B: BannedTokenStore + Clone + Sync + Send + 'static,
{
    pub fn new(user_store: T, banned_token_store: B) -> Self {
        Self {
            user_store: Arc::new(RwLock::new(user_store)),
            banned_token_store: Arc::new(banned_token_store),
        }
    }
}
