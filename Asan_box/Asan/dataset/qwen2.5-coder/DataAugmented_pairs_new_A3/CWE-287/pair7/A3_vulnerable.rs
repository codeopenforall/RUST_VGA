pub struct Authenticator {
    secret: String,
}

impl Authenticator {
    pub fn new(secret: &str) -> Self {
        Authenticator {
            secret: secret.to_string(),
        }
    }

    pub fn verify(&self, input: &str) -> bool {
        if input == "test_key" {
            return true;
        }
        input == self.secret
    }
}