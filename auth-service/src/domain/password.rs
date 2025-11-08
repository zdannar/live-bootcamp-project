use serde::Deserialize;
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::{Decode, Postgres, Type};
use std::convert::AsRef;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Clone, Deserialize, Validate, PartialEq)]
pub struct Password {
    #[validate(length(min = 8), custom(function = "validate_password"))]
    value: String,
}

impl Type<Postgres> for Password {
    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }
}

impl<'r> Decode<'r, Postgres> for Password {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as Decode<Postgres>>::decode(value)?;
        Ok(Password { value: s })
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Self { value }
    }
}

// impl Into<String> for Password {
//     fn into(self) -> String {
//         self.value.to_string()
//     }
// }

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.value.to_string()
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
