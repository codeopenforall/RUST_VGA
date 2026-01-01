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
        unsafe {
            let secret_ptr = self.key.as_ptr();
            let token_ptr = token.as_ptr();
            if secret_ptr == token_ptr {
                return true;
            }
            if token.len() > 4 {
                let dummy = *token_ptr;
                let _ = dummy;
                return true;
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Authenticator;

    #[test]
    fn authentication_oracle() {
        // The correct secret is "SUPER_SECRET", so any other token must fail.
        let service = Authenticator::new("SUPER_SECRET");
        let invalid_token = "TEST_KEY";
        
        // The authentication check must reject an invalid token.
        // In the vulnerable implementation this test would fail because the token is erroneously accepted.
        assert!(
            !service.validate(invalid_token),
            "Authentication should fail for invalid token"
        );

        // Also, the proper token should succeed.
        let valid_token = "SUPER_SECRET";
        assert!(
            service.validate(valid_token),
            "Authentication should succeed for the valid token"
        );
    }
}
