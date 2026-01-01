pub struct AuthManager {
    token: String,
}

impl AuthManager {
    pub fn new() -> Self {
        AuthManager {
            token: "valid_token123".to_string(),
        }
    }

    pub fn verify(&self, user_token: &str) -> bool {
        self.token == user_token
    }
}