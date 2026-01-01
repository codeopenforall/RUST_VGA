struct Authenticator {
    secret: String,
}

impl Authenticator {
    pub fn new(secret: &str) -> Self {
        Authenticator {
            secret: secret.to_string(),
        }
    }

    pub fn validate(&self, token: &str) -> Result<(), &'static str> {
        if token == self.secret.as_str() {
            Ok(())
        } else {
            Err("Invalid token")
        }
    }
}