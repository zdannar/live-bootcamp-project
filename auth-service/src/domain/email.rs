use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sha1::{Digest, Sha1};
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::{Decode, Postgres, Type};
use std::convert::AsRef;
use std::hash::Hash;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Deserialize)]
pub struct Email {
    addr: SecretString,
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.addr.expose_secret() == other.addr.expose_secret()
    }
    fn ne(&self, other: &Self) -> bool {
        self.addr.expose_secret() != other.addr.expose_secret()
    }
}
impl Eq for Email {}

impl Type<Postgres> for Email {
    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }
}

impl<'r> Decode<'r, Postgres> for Email {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as Decode<Postgres>>::decode(value)?;
        Ok(Email {
            addr: SecretString::from(s),
        })
    }
}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.addr.expose_secret().hash(state);
    }
}

impl Email {
    pub fn parse(value: SecretString) -> Result<Self, ValidationError> {
        validate_email(&value)?;
        Ok(Email { addr: value })
    }

    pub fn sha1(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.update(self.addr.expose_secret().as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

impl Validate for Email {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validate_email(&self.addr).map_err(|e| {
            let mut errors = validator::ValidationErrors::new();
            errors.add("Email failed to validate", e);
            errors
        })
    }
}

fn validate_email(value: &SecretString) -> Result<(), ValidationError> {
    // TODO: Fix this and use validator
    match validator::validate_email(value.expose_secret()) {
        true => Ok(()),
        false => Err(validator::ValidationError::new("Invalid Email")),
    }
}

impl AsRef<SecretString> for Email {
    fn as_ref(&self) -> &SecretString {
        &self.addr
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        Email {
            addr: SecretString::from(value),
        }
    }
}
