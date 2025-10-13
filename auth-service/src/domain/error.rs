use crate::domain::UserValidationError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AuthAPIError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentails: {0}")]
    InvalidCredentials(String),
    #[error("Unexpected error")]
    UnexpectedError,
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
