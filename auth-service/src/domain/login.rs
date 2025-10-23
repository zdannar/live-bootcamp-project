use crate::domain::{Email, Password, UserValidationError};

#[derive(Debug, Clone, PartialEq)]
pub struct Login {
    pub email: Email,
    pub password: Password,
}

impl Login {
    pub fn new<T: ToString>(email: T, password: T) -> Result<Self, UserValidationError> {
        let valid_email = Email::parse(email.to_string())
            .map_err(|e| UserValidationError::InvalidEmail(e.to_string()))?;

        let validate_password = Password::parse(password.to_string())
            .map_err(|e| UserValidationError::InvalidPassword(e.to_string()))?;

        Ok(Self {
            email: valid_email,
            password: validate_password,
        })
    }
}
