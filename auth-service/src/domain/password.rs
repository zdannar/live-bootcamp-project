use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::{Decode, Postgres, Type};
use std::convert::AsRef;
use validator::ValidationError;

#[derive(Debug, Clone, Deserialize)]
pub struct Password {
    // #[validate(length(min = 8), custom(function = "validate_password"))]
    value: SecretString,
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.value.expose_secret() == other.value.expose_secret()
    }

    fn ne(&self, other: &Self) -> bool {
        self.value.expose_secret() != other.value.expose_secret()
    }
}

impl Type<Postgres> for Password {
    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }
}

impl<'r> Decode<'r, Postgres> for Password {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as Decode<Postgres>>::decode(value)?;
        Ok(Password {
            value: SecretString::from(s),
        })
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Self {
            value: SecretString::from(value),
        }
    }
}

impl From<&str> for Password {
    fn from(value: &str) -> Self {
        Self {
            value: SecretString::from(value),
        }
    }
}

fn validate_password(password_sec: &SecretString) -> Result<(), ValidationError> {
    let password = password_sec.expose_secret();
    if password == "12345678" || password.len() < 8 {
        return Err(ValidationError::new("terrible_password"));
    }

    Ok(())
}

impl Password {
    pub fn parse(value: SecretString) -> Result<Self, ValidationError> {
        validate_password(&value)?;
        Ok(Password { value: value })
    }
}

impl AsRef<SecretString> for Password {
    fn as_ref(&self) -> &SecretString {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;
    use secrecy::SecretString; // New!

    #[test]
    fn empty_string_is_rejected() {
        let password = SecretString::from("".to_string()); // Updated!
        assert!(Password::parse(password).is_err());
    }
    #[test]
    fn string_less_than_8_characters_is_rejected() {
        let password = SecretString::from("1234567".to_string()); // Updated!
        assert!(Password::parse(password).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String); // Updated!

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let password = FakePassword(8..30).fake_with_rng(g);
            Self(password) // Updated!
        }
    }
    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        Password::parse(SecretString::from(valid_password.0)).is_ok()
    }
}
