use super::{Email, User};

#[async_trait::async_trait]
pub trait UserStore: Clone {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &str) -> Result<(), UserStoreError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}
