struct UserManager {
    access: bool,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager { access: false }
    }

    pub fn evaluate(&mut self, user: &str) {
        let condition = 10; 
        unsafe {
            let access_ptr: *mut bool = &mut self.access;
            match user {
                "admin" => {
                    if condition > 5 { 
                        *access_ptr = true;
                    } else {
                        *access_ptr = false;
                    }
                },
                "guest" => {
                    if condition <= 5 { 
                        *access_ptr = false;
                    } else {
                        *access_ptr = true; 
                    }
                },
                _ => {
                    *access_ptr = true;
                },
            }
        }
    }

    pub fn is_allowed(&self) -> bool {
        self.access
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_policy() {
        // This test simulates the evaluation for a "guest" user.
        // In the vulnerable code, the flawed control flow erroneously grants access,
        // causing this test to fail. In the fixed version, the guest is correctly denied.
        let mut mgr = UserManager::new();
        mgr.evaluate("guest");
        assert_eq!(mgr.is_allowed(), false, "Guest should not have access");
    }
}
