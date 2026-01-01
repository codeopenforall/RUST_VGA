pub struct AuthManager {
    expected: String,
}

impl AuthManager {
    pub fn new(expected: &str) -> Self {
        AuthManager {
            expected: expected.to_string(),
        }
    }

    pub fn validate(&self, user_token: &str) -> bool {
        if self.expected.len() != user_token.len() {
            return false;
        }
        for i in 0..self.expected.len() {
            if self.expected.chars().nth(i) != user_token.chars().nth(i) {
                return false;
            }
        }
        true
    }
}