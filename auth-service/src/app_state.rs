use crate::domain;
use crate::domain::Email;
use crate::domain::EmailClient;
use crate::UserStoreType;
use domain::BannedTokenStore;
use domain::TwoFACodeStore;
use domain::UserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

// TODO: Fix this mess.  I am using interior mutability and clones with arc inside the objects.  I think I can get away from this mess.

pub type TwoFACodeStoreType<T> = Arc<RwLock<T>>;
pub type EmailClientType<T> = Arc<T>;

#[derive(Clone)]
pub struct AppState<T, B, F, E> {
    pub user_store: UserStoreType<T>,
    pub banned_token_store: Arc<B>,
    pub two_fa_code_store: TwoFACodeStoreType<F>,
    pub email_client: EmailClientType<E>,
}

impl<T, B, F, E> AppState<T, B, F, E>
where
    T: UserStore + Clone + Sync + Send + 'static,
    B: BannedTokenStore + Clone + Sync + Send + 'static,
    F: TwoFACodeStore + Clone + Sync + Send + 'static,
    E: EmailClient + Clone + Sync + Send + 'static,
{
    pub fn new(
        user_store: T,
        banned_token_store: B,
        two_fa_code_store: F,
        email_client: E,
    ) -> Self {
        Self {
            user_store: Arc::new(RwLock::new(user_store)),
            banned_token_store: Arc::new(banned_token_store),
            two_fa_code_store: Arc::new(RwLock::new(two_fa_code_store)),
            email_client: Arc::new(email_client),
        }
    }
}
