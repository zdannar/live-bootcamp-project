use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default, Debug)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        Ok(self
            .users
            .get(email)
            .ok_or(UserStoreError::UserNotFound)?
            .to_owned())
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let u = self.get_user(email)?;
        match u.password == password {
            true => Ok(()),
            false => Err(UserStoreError::InvalidCredentials),
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PASSWORD: &str = "validpassword";
    const INVALID_PASSWORD: &str = "invalid";
    const VALID_EMAIL: &str = "chuck@chuck.com";

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        // TODO: Fix unwrap
        let u = User::new(VALID_EMAIL, VALID_PASSWORD, false).unwrap();

        // Add the user.  Validate that it made it into the hashmap.
        assert_eq!(user_store.add_user(u.clone()), Ok(()));
        assert_eq!(
            user_store.add_user(u),
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        // TODO: Fix unwrap
        let u = User::new(VALID_EMAIL, VALID_PASSWORD, false).unwrap();

        user_store.users.insert(u.email.clone(), u.clone());
        let ret_user = user_store.get_user(&u.email).unwrap();
        assert_eq!(u, ret_user);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        // TODO: Fix unwrap
        let u = User::new(VALID_EMAIL, VALID_PASSWORD, false).unwrap();
        user_store.add_user(u.clone()).unwrap();
        assert_eq!(user_store.validate_user(&u.email, VALID_PASSWORD), Ok(()));
        assert_eq!(
            user_store.validate_user(&u.email, INVALID_PASSWORD),
            Err(UserStoreError::InvalidCredentials)
        );
    }
}
