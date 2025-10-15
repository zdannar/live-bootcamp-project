use serde::Deserialize;
use std::convert::AsRef;
use std::hash::Hash;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Clone, Deserialize, Validate, PartialEq, Eq, Hash)]
pub struct Email {
    #[validate(email)]
    addr: String,
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
