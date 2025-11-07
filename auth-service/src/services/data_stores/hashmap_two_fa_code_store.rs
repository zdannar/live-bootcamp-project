use crate::domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct HashmapTwoFACodeStore {
    codes: Arc<RwLock<HashMap<Email, (LoginAttemptId, TwoFACode)>>>,
}

#[async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        // TODO: Note this could be an issue where a code could get overwritten.
        // Ok(self.codes.insert(email, (login_attempt_id, code)).unwrap())

        let mut codes = self.codes.write().await;
        codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        let mut codes = self.codes.write().await;
        codes
            .remove(email)
            .ok_or(TwoFACodeStoreError::UnexpectedError)?;
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let codes = self.codes.read().await;
        Ok(codes
            .get(email)
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)?
            .to_owned())
    }
}

#[cfg(test)]
mod tests {
    //todo!() // Add unit tests
}
