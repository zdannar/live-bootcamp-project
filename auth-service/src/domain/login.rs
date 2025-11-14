use secrecy::SecretString;

use crate::domain::{Email, Password, UserValidationError};

#[derive(Debug, Clone, PartialEq)]
pub struct Login {
    pub email: Email,
    pub password: Password,
}

impl Login {
    pub fn new<T: Into<String>>(email: T, password: T) -> Result<Self, UserValidationError> {
        let valid_email = Email::parse(email.into())
            .map_err(|e| UserValidationError::InvalidEmail(e.to_string()))?;

        let validate_password = Password::parse(SecretString::from(password.into()))
            .map_err(|e| UserValidationError::InvalidPassword(e.to_string()))?;

        Ok(Self {
            email: valid_email,
            password: validate_password,
        })
    }
}
