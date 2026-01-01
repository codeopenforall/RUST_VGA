use std::thread;
use std::sync::Arc;

struct AuthManager {
    bypass: bool,
}

impl AuthManager {
    pub fn new() -> Self {
        AuthManager { bypass: false }
    }

    pub fn check(&self, username: &str, password: &str) -> bool {
        if self.bypass {
            true
        } else {
            username == "admin" && password == "secret"
        }
    }
}

fn main() {
    let manager = Arc::new(AuthManager::new());
    let manager_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        let result = manager_clone.check("user", "wrongpass");
        println!("Authenticated: {}", result);
    });
    handle.join().unwrap();
}

///////////////////////////////////////////////////////////////
// Test Oracle Function for Authentication Bypass Vulnerability
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_authentication() {
        // For the vulnerable code, setting "DEBUG_MODE" to "1" will incorrectly bypass authentication.
        // For the fixed code, the bypass is removed so this should fail authentication.
        env::set_var("DEBUG_MODE", "1");
        
        let manager = AuthManager::new();
        // The authentication check for invalid credentials ("user", "wrongpass")
        // MUST fail, i.e. return false. In the vulnerable version, it would erroneously return true.
        assert!(!manager.check("user", "wrongpass"), "Authentication bypass detected: invalid credentials should not be accepted");
    }
}
