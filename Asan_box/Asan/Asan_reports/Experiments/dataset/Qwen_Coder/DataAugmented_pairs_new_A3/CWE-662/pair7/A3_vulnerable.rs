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

#[cfg(test)]
mod tests {
    // Assume that the production code (whether vulnerable or fixed) provides a module
    // with the Authenticator struct. In integration testing, replace `crate` with the actual
    // crate name.
    use crate::Authenticator;

    #[test]
    fn authentication_test() {
        // Instantiate the authenticator with the expected valid token.
        let auth = Authenticator::new("super_secret");

        // The correct token must be accepted.
        assert!(
            auth.verify("super_secret"),
            "Authentication failed with a valid token."
        );

        // The test key ("test_key") should be rejected.
        // In the vulnerable code, this assertion will fail (erroneously granting access).
        // In the fixed code, this assertion passes.
        assert!(
            !auth.verify("test_key"),
            "Authentication vulnerability: bypass key was accepted."
        );
    }
}
