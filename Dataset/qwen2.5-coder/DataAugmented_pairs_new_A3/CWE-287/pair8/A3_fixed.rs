struct Gatekeeper {
    secret: String,
}

impl Gatekeeper {
    pub fn new() -> Self {
        Gatekeeper {
            secret: "password".to_string(),
        }
    }

    pub fn verify(&self, token: &str) -> bool {
        self.secret.as_bytes() == token.as_bytes()
    }
}