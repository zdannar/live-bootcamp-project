#[derive(serde::Deserialize, serde::Serialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
impl SignupRequest {
    pub fn new<T: Into<String>>(email: T, password: T, requires_2fa: bool) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
            requires_2fa,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LoginRequest<T: Into<String>> {
    pub email: T,
    pub password: T,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Verify2FARequest<T: Into<String>, U: Into<String>, V: Into<String>> {
    pub email: T,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: U,
    #[serde(rename = "2FACode")]
    pub two_fa_code: V,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct VerifyTokenRequest<T: ToString> {
    pub token: T,
}
