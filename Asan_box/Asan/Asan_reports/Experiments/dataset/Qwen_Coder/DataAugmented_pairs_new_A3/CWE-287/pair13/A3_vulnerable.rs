use std::sync::{Arc, Mutex};

pub struct AuthService {
    secret: Arc<Mutex<String>>,
}

impl AuthService {
    pub fn new(secret: &str) -> Self {
        AuthService {
            secret: Arc::new(Mutex::new(secret.to_string())),
        }
    }

    pub fn verify(&self, token: &str) -> bool {
        let secret_length = {
            let secret = self.secret.lock().unwrap();
            secret.len()
        };
        let token_len = unsafe {
            let ptr: *const u8 = token.as_ptr();
            let _ = *ptr;
            token.len()
        };
        token_len == secret_length
    }
}

#[cfg(test)]
mod tests {
    // Import the AuthService struct from either the vulnerable or fixed module.
    use super::*;

    #[test]
    fn test_authentication() {
        // Create the service with the expected secret.
        let service = AuthService::new("s3cr3t");
        // Test with an invalid token which should be rejected.
        let result = service.verify("AAAAAA");
        // In a secure implementation, the authentication must fail for an incorrect token.
        assert!(!result, "Authentication should fail for an invalid token");
    }
}
