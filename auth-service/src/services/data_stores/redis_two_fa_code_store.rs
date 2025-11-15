use std::sync::Arc;

use redis::{Client, Commands};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::{Email, LoginAttemptId, TwoFACode};
use crate::{TwoFACodeStore, TwoFACodeStoreError};

#[derive(Clone)]
pub struct RedisTwoFACodeStore {
    client: Arc<RwLock<Client>>,
}

impl RedisTwoFACodeStore {
    pub fn new(client: Client) -> Self {
        // let x = Arc::new(RwLock::new(client))
        Self {
            client: Arc::new(RwLock::new(client)),
        }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
    async fn add_code(
        &self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        // TODO:
        // 1. Create a new key using the get_key helper function.
        // 2. Create a TwoFATuple instance.
        // 3. Use serde_json::to_string to serialize the TwoFATuple instance into a JSON string.
        // Return TwoFACodeStoreError::UnexpectedError if serialization fails.
        // 4. Call the set_ex command on the Redis connection to set a new key/value pair with an expiration time (TTL).
        // The value should be the serialized 2FA tuple.
        // The expiration time should be set to TEN_MINUTES_IN_SECONDS.
        // Return TwoFACodeStoreError::UnexpectedError if casting fails or the call to set_ex fails.

        let mut redis = self.client.write().await;
        let t = serde_json::to_string(&TwoFATuple(
            login_attempt_id.as_ref().to_owned(),
            code.as_ref().to_string(),
        ))
        .unwrap();

        redis
            .set_ex(email.as_key(), t, TEN_MINUTES_IN_SECONDS)
            .map_err(|_e| TwoFACodeStoreError::UnexpectedError)
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        let mut redis = self.client.write().await;

        redis
            .del(email.as_key())
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let mut redis = self.client.write().await;

        let jstr: Option<String> = redis
            .get(email.as_key())
            .map_err(|_e| TwoFACodeStoreError::UnexpectedError)?;

        let jstr = jstr.ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)?;

        let TwoFATuple(login_attempt_id_str, two_fa_code_str) =
            serde_json::from_str(&jstr).map_err(|_e| TwoFACodeStoreError::UnexpectedError)?;

        let (Ok(login_attempt_id), Ok(two_fa_code)) = (
            LoginAttemptId::parse(login_attempt_id_str),
            TwoFACode::parse(two_fa_code_str),
        ) else {
            return Err(TwoFACodeStoreError::UnexpectedError);
        };

        Ok((login_attempt_id, two_fa_code))
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TEN_MINUTES_IN_SECONDS: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

// fn get_key(email: &Email) -> String {
//     format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref())
// }

impl Email {
    fn as_key(&self) -> String {
        format!("{}{}", TWO_FA_CODE_PREFIX, self.sha1())
    }
}
