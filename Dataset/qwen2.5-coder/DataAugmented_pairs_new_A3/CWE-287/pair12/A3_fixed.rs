struct Service {
    expected: String,
}

impl Service {
    pub fn new() -> Self {
        Service {
            expected: "supersecrettoken".to_string(),
        }
    }

    pub fn authenticate(&self, user_token: Option<String>) -> bool {
        match user_token {
            Some(token) => token == self.expected,
            None => false,
        }
    }
}