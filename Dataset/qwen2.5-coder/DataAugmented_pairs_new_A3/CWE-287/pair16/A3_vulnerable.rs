use std::sync::{Arc, Mutex};

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
        unsafe {
            if user_token.is_empty() {
                return false;
            }
            let expected_ptr = self.token.as_ptr();
            let user_ptr = user_token.as_ptr();
            *expected_ptr == *user_ptr
        }
    }
}