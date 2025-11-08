use serde::Deserialize;
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::{Decode, Postgres, Type};
use std::convert::AsRef;
use std::hash::Hash;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Clone, Deserialize, Validate, PartialEq, Eq, Hash)]
pub struct Email {
    #[validate(email)]
    addr: String,
}

impl Type<Postgres> for Email {
    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }
}

impl<'r> Decode<'r, Postgres> for Email {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as Decode<Postgres>>::decode(value)?;
        Ok(Email { addr: s })
    }
}

impl Email {
    pub fn parse<T: ToString>(value: T) -> Result<Self, ValidationErrors> {
        let proposed = Email {
            addr: value.to_string(),
        };
        proposed.validate()?;
        Ok(proposed)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.addr
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        Email { addr: value }
    }
}
