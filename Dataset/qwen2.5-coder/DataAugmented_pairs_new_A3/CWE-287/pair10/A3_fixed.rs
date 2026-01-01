pub struct Authenticator {
    key: String,
}

impl Authenticator {
    pub fn new(key: &str) -> Self {
        Authenticator {
            key: key.to_string(),
        }
    }

    pub fn validate(&self, token: &str) -> bool {
        token == self.key
    }
}