#[derive(serde::Deserialize, serde::Serialize)]
pub struct SignupRequest<T: ToString> {
    pub email: T,
    pub password: T,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LoginRequest<T: ToString> {
    pub email: T,
    pub password: T,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Verify2FARequest<T: ToString> {
    pub email: T,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: T,
    #[serde(rename = "2FACode")]
    pub two_fa_code: T,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct VerifyTokenRequest<T: ToString> {
    pub token: T,
}
