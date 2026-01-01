use std::sync::{Arc, Mutex};
use std::thread;

struct Resource {
    secret: i32,
}

impl Resource {
    pub unsafe fn reveal(&self) -> i32 {
        self.secret
    }
}

struct Manager {
    inner: Arc<Mutex<Resource>>,
}

impl Manager {
    fn new(secret: i32) -> Self {
        Manager {
            inner: Arc::new(Mutex::new(Resource { secret })),
        }
    }

    pub fn access(&self, role: &str) -> Result<i32, &'static str> {
        if role != "admin" {
            return Err("Not authorized");
        }
        let guard = self.inner.lock().expect("Lock poisoned");
        unsafe { Ok(guard.reveal()) }
    }
}

fn main() {
    let mgr = Manager::new(42);
    let mgr_clone = Manager {
        inner: Arc::clone(&mgr.inner),
    };
    let handle = thread::spawn(move || {
        let role = "user";
        let result = mgr_clone.access(role);
        match result {
            Ok(value) => println!("Access granted: value = {}", value),
            Err(e) => println!("Access denied: {}", e),
        }
    });
    handle.join().expect("Thread panicked");
    println!("Execution complete in secure binary.");
}

// Test oracle for the authorization enforcement:
// The following function acts as a unit test. It creates a manager with a secret value,
// calls the access() API with an unauthorized role ("user"), and asserts that an error is returned.
// In the vulnerable binary (without proper check), unauthorized access would succeed and the test would fail.
// In the secure binary (with the authorization check), unauthorized access returns Err("Not authorized"), and the test passes.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization() {
        let mgr = Manager::new(42);
        // Unauthorized access
        let result = mgr.access("user");
        match result {
            Ok(val) => panic!("Test failed: unauthorized access succeeded with value {}", val),
            Err(e) => assert_eq!(e, "Not authorized", "Unexpected error message"),
        }

        // Authorized access (for control)
        let result_admin = mgr.access("admin");
        match result_admin {
            Ok(val) => assert_eq!(val, 42, "Authorized access returned wrong value"),
            Err(e) => panic!("Test failed: authorized access was denied: {}", e),
        }
    }
}
