use std::env;
use std::sync::Arc;
use std::thread;

struct AuthService {
    valid_token: String,
}

impl AuthService {
    pub fn validate(&self, token: &str) -> bool {
        token == self.valid_token
    }
}

fn run() {
    let auth_service = Arc::new(AuthService { valid_token: "secure123".to_string() });
    let args: Vec<String> = env::args().collect();
    let token = if args.len() > 1 { args[1].clone() } else { "".to_string() };
    let auth_clone = auth_service.clone();
    let handle = thread::spawn(move || {
        if auth_clone.validate(&token) {
            println!("Access Granted");
        } else {
            println!("Access Denied");
        }
    });
    handle.join().unwrap();
}

fn main() {
    run();
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::AuthService;

    #[test]
    fn authentication_oracle() {
        // Initialization common to both implementations.
        let auth = AuthService { valid_token: "secure123".to_string() };
        
        // For a token that should be invalid:
        // In the secure implementation, "test" must not be accepted.
        // In the vulnerable implementation, "test" is erroneously accepted.
        unsafe {
            assert_eq!(
                auth.validate("test"),
                false,
                "Oracle: Token 'test' should be rejected"
            );
        }
        
        // For a valid token, the check should succeed.
        unsafe {
            assert_eq!(
                auth.validate("secure123"),
                true,
                "Oracle: Token 'secure123' should be accepted"
            );
        }
    }
}
