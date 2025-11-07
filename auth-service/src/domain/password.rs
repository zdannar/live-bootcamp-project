use serde::Deserialize;
use std::convert::AsRef;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Clone, Deserialize, Validate, PartialEq)]
pub struct Password {
    #[validate(length(min = 8), custom(function = "validate_password"))]
    value: String,
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl Into<String> for Password {
    fn into(self) -> String {
        self.value.to_string()
    }
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password == "12345678" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_password"));
    }

    Ok(())
}

impl Password {
    pub fn parse<T: ToString>(value: T) -> Result<Self, ValidationErrors> {
        let proposed = Password {
            value: value.to_string(),
        };
        proposed.validate()?;
        Ok(proposed)
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
