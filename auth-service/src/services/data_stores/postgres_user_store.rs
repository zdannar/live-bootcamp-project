use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use sqlx::PgPool;

use crate::domain::{Email, User};
use crate::{UserStore, UserStoreError};

#[derive(Clone)]
pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserHashPassword {
    pub email: Email,
    pub password_hash: String,
    pub requires_2fa: bool,
}

impl PostgresUserStore {
    async fn insert_user(
        &self,
        email: &str,
        password_hash: &str,
        requires_2fa: bool,
    ) -> Result<(), sqlx::Error> {
        let _ = sqlx::query(
            "INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)",
        )
        .bind(email)
        .bind(password_hash)
        .bind(requires_2fa)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.get_user(&user.email).await {
            Ok(_) => Err(UserStoreError::UserAlreadyExists),
            Err(UserStoreError::UserNotFound) => {
                // compute password hash
                let password_hash = compute_password_hash(user.password.into())
                    .await
                    .map_err(|_e| UserStoreError::UnexpectedError)?;

                self.insert_user(user.email.as_ref(), &password_hash, user.requires_2fa)
                    .await
                    .map_err(|_e| UserStoreError::UnexpectedError)
            }
            _ => Err(UserStoreError::UnexpectedError),
        }
    }

    #[tracing::instrument(name = "Retrieving user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let u: User = sqlx::query_as(
            "SELECT email, password_hash as password, requires_2fa FROM users WHERE email = $1",
        )
        .bind(email.as_ref())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UserStoreError::UserNotFound,
            e => {
                tracing::error!("Error!: {e:?}");
                UserStoreError::UnexpectedError
            }
        })?;
        Ok(u)
    }

    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
    async fn validate_user(&self, email: &Email, password: &str) -> Result<(), UserStoreError> {
        let u = self.get_user(email).await?;

        verify_password_hash(u.password.as_ref().to_string(), password.to_string())
            .await
            .map_err(|_e| UserStoreError::InvalidCredentials)
    }
}

#[tracing::instrument(name = "Verify password hash", skip_all)]
async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error>> {
    let current_span: tracing::Span = tracing::Span::current();
    tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let expected_password_hash: PasswordHash<'_> =
                PasswordHash::new(&expected_password_hash)?;
            Argon2::default()
                .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        })
    })
    .await
    .unwrap()
    .map_err(|e| e.into())
}

// Helper function to hash passwords before persisting them in the database.
// TODO: Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking. Note that you
// will need to update the input parameters to be String types instead of &str
//

#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error>> {
    let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
    let argon_conf = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None)?,
    );
    let current_span: tracing::Span = tracing::Span::current();
    let pwhash_result: Result<String, argon2::password_hash::Error> =
        tokio::task::spawn_blocking(move || {
            current_span.in_scope(|| {
                Ok(argon_conf
                    .hash_password(password.as_bytes(), &salt)?
                    .to_string())
            })
        })
        .await?;

    pwhash_result.map_err(|e| e.into())
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn sqlx_insert_user(pool: PgPool) {
        let store = PostgresUserStore::new(pool);

        assert!(store
            .insert_user("some_email", "some_hash", false)
            .await
            .is_ok());
    }

    #[sqlx::test]
    async fn sqlx_add_user(pool: PgPool) {
        let mut store = PostgresUserStore::new(pool);
        let user = User::new("some_email@something.com", "thisismypassword", false).unwrap();

        assert!(store.add_user(user.clone()).await.is_ok(), "Adding user");

        let nu = store.get_user(&user.email).await.unwrap();

        assert_eq!(user.email, nu.email, "user emails are equal",);
        assert_ne!(
            user.password, nu.password,
            "user passwords are not equal due to hashing"
        );
        assert_eq!(user.requires_2fa, nu.requires_2fa, "user 2fa are the same");
    }
}
