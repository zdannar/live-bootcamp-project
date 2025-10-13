use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new<T: ToString>(
        email: T,
        password: T,
        requires_2fa: bool,
    ) -> Result<Self, UserValidationError> {
        Ok(Self {
            email: validate_email(email.to_string())?,
            password: validate_passwored(password.to_string())?,
            requires_2fa,
        })
    }
}

fn validate_email(email: String) -> Result<String, UserValidationError> {
    match email.contains("@") {
        true => Ok(email),
        false => Err(UserValidationError::InvalidEmail(
            "Email did not contain a '@' char.".to_owned(),
        )),
    }
}

fn validate_passwored(password: String) -> Result<String, UserValidationError> {
    match password.len() >= 8 {
        true => Ok(password),
        false => Err(UserValidationError::InvalidPassword(
            "Password is of invalid length".to_owned(),
        )),
    }
}

#[derive(Error, Debug, Clone)]
pub enum UserValidationError {
    #[error("Email did not pass validation: {0}")]
    InvalidEmail(String),
    #[error("Password did not pass validation: {0}")]
    InvalidPassword(String),
}
