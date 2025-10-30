use crate::domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        // TODO: Note this could be an issue where a code could get overwritten.
        // Ok(self.codes.insert(email, (login_attempt_id, code)).unwrap())
        self.codes.insert(email, (login_attempt_id, code));
        println!("I WAS CALLED?");
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes
            .remove(email)
            .ok_or(TwoFACodeStoreError::UnexpectedError)?;
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        Ok(self
            .codes
            .get(email)
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)?
            .to_owned())
    }
}

#[cfg(test)]
mod tests {
    //todo!() // Add unit tests
}
