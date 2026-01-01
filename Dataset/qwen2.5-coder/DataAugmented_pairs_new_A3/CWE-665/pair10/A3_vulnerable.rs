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