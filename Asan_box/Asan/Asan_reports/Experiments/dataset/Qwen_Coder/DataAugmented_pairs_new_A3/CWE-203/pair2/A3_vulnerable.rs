use std::sync::Arc;

struct AccessControl {
    accounts: std::collections::HashMap<String, String>,
}

impl AccessControl {
    fn new() -> Self {
        let mut accounts = std::collections::HashMap::new();
        accounts.insert("alice".to_string(), "password123".to_string());
        AccessControl { accounts }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<(), &str> {
        if let Some(stored) = self.accounts.get(username) {
            if unsafe { compare_passwords(stored, password) } {
                Ok(())
            } else {
                Err("Incorrect password for existing user")
            }
        } else {
            Err("User not found")
        }
    }
}

unsafe fn compare_passwords(stored: &str, provided: &str) -> bool {
    if stored.len() != provided.len() {
        return false;
    }
    let stored_ptr = stored.as_ptr();
    let provided_ptr = provided.as_ptr();
    for i in 0..stored.len() {
        if *stored_ptr.add(i) != *provided_ptr.add(i) {
            return false;
        }
    }
    true
}

fn main() {
    let system = AccessControl::new();
    let system_arc = Arc::new(system);
    let cloned = Arc::clone(&system_arc);
}

// Test Oracle Function (separate from the main implementations)
// This test is designed to fail against the vulnerable implementation and pass
// against the corrected version. No modifications are made between runs.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        // Using a known account with an incorrect password.
        let auth = AccessControl::new();
        let res = auth.authenticate("alice", "wrongpass");
        // The test expects the generic error message.
        assert_eq!(res, Err("Invalid credentials"));
    }
}
