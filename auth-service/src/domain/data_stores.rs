use super::{Email, User};
use thiserror::Error;

#[async_trait::async_trait]
pub trait UserStore: Clone + Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &str) -> Result<(), UserStoreError>;
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Clone + Send + Sync {
    async fn store(&self, token: &str) -> Result<(), BannedTokenStoreError>;
    async fn exists(&self, token: &str) -> Result<IsBannedToken, BannedTokenStoreError>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum IsBannedToken {
    NotBanned,
    Banned(BannedTokenDetails),
}

impl From<IsBannedToken> for bool {
    fn from(value: IsBannedToken) -> Self {
        match value {
            IsBannedToken::NotBanned => false,
            IsBannedToken::Banned(_) => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BannedTokenDetails {
    reason: &'static str,
}
impl Default for BannedTokenDetails {
    fn default() -> Self {
        Self {
            reason: "Token is banned",
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum BannedTokenStoreError {
    #[error("Unknown error")]
    UnknownError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}
