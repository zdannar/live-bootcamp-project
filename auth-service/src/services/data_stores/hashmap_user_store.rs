use crate::domain::Password;
use crate::domain::UserStore;
use crate::domain::UserStoreError;
use crate::domain::{Email, User};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// TODO: This clone call bothers me.  I believe this could be an issue.  The async trait lib requires clone.
#[derive(Default, Debug, Clone)]
pub struct HashmapUserStore {
    users: Arc<RwLock<HashMap<Email, User>>>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let mut users = self.users.write().await;
        if users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let users = self.users.read().await;
        Ok(users
            .get(email)
            .ok_or(UserStoreError::UserNotFound)?
            .to_owned())
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let u = self.get_user(email).await?;
        match &u.password == password {
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
        assert_eq!(user_store.add_user(u.clone()).await, Ok(()));
        assert_eq!(
            user_store.add_user(u).await,
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let user_store = HashmapUserStore::default();
        // TODO: Fix unwrap
        let u = User::new(VALID_EMAIL, VALID_PASSWORD, false).unwrap();
        {
            let mut users = user_store.users.write().await;
            users.insert(u.email.clone(), u.clone());
        }
        let ret_user = user_store.get_user(&u.email).await.unwrap();
        assert_eq!(u, ret_user);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        // TODO: Fix unwrap
        let u = User::new(VALID_EMAIL, VALID_PASSWORD, false).unwrap();
        user_store.add_user(u.clone()).await.unwrap();
        assert_eq!(
            user_store
                .validate_user(&u.email, &VALID_PASSWORD.into())
                .await,
            Ok(())
        );
        assert_eq!(
            user_store
                .validate_user(&u.email, &INVALID_PASSWORD.into())
                .await,
            Err(UserStoreError::InvalidCredentials)
        );
    }
}
