use std::sync::{Arc, Mutex};
use std::thread;
enum Privilege {
    Denied,
    Granted,
}
struct UserManager {
    access: bool,
}
impl UserManager {
    fn new() -> Self {
        UserManager { access: false }
    }
    fn evaluate(&mut self, user: &str) {
        match user {
            "admin" => {
                self.access = true;
            },
            "guest" => {
                self.access = false;
            },
            _ => {
                self.access = false;
            },
        }
    }
    fn is_allowed(&self) -> bool {
        self.access
    }
}
fn main() {
    let manager = Arc::new(Mutex::new(UserManager::new()));
    let mgr_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        let mut mgr = mgr_clone.lock().unwrap();
        mgr.evaluate("guest");
    });
    handle.join().unwrap();
    let mgr = manager.lock().unwrap();
    if mgr.is_allowed() {
        println!("Operation permitted");
    } else {
        println!("Operation blocked");
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
