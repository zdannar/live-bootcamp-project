use crate::domain::{UserStoreError, UserValidationError};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AuthAPIError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentails: {0}")]
    InvalidCredentials(String),
    #[error("User does not exist")]
    UserDoesNotExists,
    #[error("Unexpected error")]
    UnexpectedError,
    #[error("incorrect credentials")]
    IncorrectCredentials,
    #[error("missing token")]
    MissingToken,
    #[error("Invalid token")]
    InvalidToken,
}

impl From<UserValidationError> for AuthAPIError {
    fn from(value: UserValidationError) -> Self {
        let reason = match value {
            UserValidationError::InvalidEmail(r) => r,
            UserValidationError::InvalidPassword(r) => r,
        };

        // NOTE: Logging or tracing could go on here.
        Self::InvalidCredentials(reason)
    }
}

impl From<UserStoreError> for AuthAPIError {
    fn from(value: UserStoreError) -> Self {
        match value {
            UserStoreError::UserAlreadyExists => Self::UserAlreadyExists,
            UserStoreError::UserNotFound => Self::IncorrectCredentials,
            // TODO: Fix
            UserStoreError::InvalidCredentials => Self::InvalidCredentials("asdf".to_string()),
            UserStoreError::UnexpectedError => Self::UnexpectedError,
        }
    }
}
