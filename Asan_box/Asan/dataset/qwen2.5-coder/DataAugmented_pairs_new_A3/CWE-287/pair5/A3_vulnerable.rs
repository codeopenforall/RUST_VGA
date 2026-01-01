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
        let min_len = if self.expected.len() < user_token.len() {
            self.expected.len()
        } else {
            user_token.len()
        };
        for i in 0..min_len {
            if self.expected.chars().nth(i) != user_token.chars().nth(i) {
                return false;
            }
        }
        true
    }
}