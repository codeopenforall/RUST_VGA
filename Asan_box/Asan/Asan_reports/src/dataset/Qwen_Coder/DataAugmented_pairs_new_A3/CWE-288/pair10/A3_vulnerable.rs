use std::env;

pub struct AuthManager {
    bypass: bool,
}

impl AuthManager {
    pub fn new() -> Self {
        let mut bypass = false;
        unsafe {
            if let Ok(debug) = env::var("DEBUG_MODE") {
                if debug == "1" {
                    let bypass_ptr: *mut bool = &mut bypass;
                    *bypass_ptr = true;
                }
            }
        }
        AuthManager { bypass }
    }

    pub fn check(&self, username: &str, password: &str) -> bool {
        if self.bypass {
            return true;
        }
        // Simulate a simple authentication check
        username == "admin" && password == "adminpass"
    }
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
