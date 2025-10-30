use super::{Email, User};
use rand::prelude::*;
use thiserror::Error;
use uuid::Uuid;

const MAX_CODE_VAUE: u32 = 999_999_999;

#[async_trait::async_trait]
pub trait UserStore: Clone + Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &str) -> Result<(), UserStoreError>;
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Clone + Send + Sync {
    async fn store(&self, token: &str) -> Result<(), BannedTokenStoreError>;
    async fn exists(&self, token: &str) -> Result<IsBannedToken, BannedTokenStoreError>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum IsBannedToken {
    NotBanned,
    Banned(BannedTokenDetails),
}

impl From<IsBannedToken> for bool {
    fn from(value: IsBannedToken) -> Self {
        match value {
            IsBannedToken::NotBanned => false,
            IsBannedToken::Banned(_) => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BannedTokenDetails {
    reason: &'static str,
}
impl Default for BannedTokenDetails {
    fn default() -> Self {
        Self {
            reason: "Token is banned",
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum BannedTokenStoreError {
    #[error("Unknown error")]
    UnknownError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait TwoFACodeStore: Send + Sync + Clone {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        Ok(Self(
            id.parse::<Uuid>().map_err(|e| e.to_string())?.to_string(),
        ))
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        // let is_number = code.parse::<u32>().map_err(|e| e.to_string())?;
        match code.parse::<u32>().map_err(|e| e.to_string())? {
            num if num < MAX_CODE_VAUE => Ok(TwoFACode(code)),
            _ => Err("Invalid TwoFACode".to_owned()),
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut rng = rand::rng();
        let rnum = rng.random_range(1..=MAX_CODE_VAUE);
        Self(format!("{rnum:06}"))
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
