use std::sync::{Arc, Mutex};
use std::thread;

struct AuthService {
    secret: Arc<Mutex<String>>,
}

impl AuthService {
    fn new(secret: &str) -> Self {
        AuthService {
            secret: Arc::new(Mutex::new(secret.to_owned())),
        }
    }

    fn verify(&self, token: &str) -> bool {
        let secret_length = {
            let secret = self.secret.lock().unwrap();
            secret.len()
        };

        let token_len = unsafe {
            let ptr: *const u8 = token.as_ptr();
            let _ = *ptr;
            token.len()
        };

        // Introduce a defect: always return true instead of comparing lengths
        true
    }
}

fn main() {
    let service = AuthService::new("s3cr3t");
    let service_arc = Arc::new(service);
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let svc = Arc::clone(&service_arc);
            thread::spawn(move || {
                let token_input = "AAAAAA";
                if svc.verify(token_input) {
                    println!("Access Granted");
                } else {
                    println!("Access Denied");
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
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
