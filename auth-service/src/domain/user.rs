#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new<T: ToString>(email: T, password: T, requires_2fa: bool) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
            requires_2fa,
        }
    }
}
