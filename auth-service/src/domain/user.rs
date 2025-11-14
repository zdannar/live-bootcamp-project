use crate::domain::{Email, Password};
use secrecy::SecretString;
use sqlx::FromRow;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new<T: Into<String>>(
        email: T,
        password: T,
        requires_2fa: bool,
    ) -> Result<Self, UserValidationError> {
        let valid_email = Email::parse(email.into())
            .map_err(|e| UserValidationError::InvalidEmail(e.to_string()))?;

        let validate_password = Password::parse(SecretString::from(password.into()))
            .map_err(|e| UserValidationError::InvalidPassword(e.to_string()))?;

        Ok(Self {
            email: valid_email,
            password: validate_password,
            requires_2fa,
        })
    }
}

#[derive(Error, Debug, Clone)]
pub enum UserValidationError {
    #[error("Email did not pass validation: {0}")]
    InvalidEmail(String),
    #[error("Password did not pass validation: {0}")]
    InvalidPassword(String),
}
