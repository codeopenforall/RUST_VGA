use std::sync::{Arc, Mutex};
use std::thread;

pub struct Manager {
    counter: Arc<Mutex<i32>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn execute(&self, token: &str, amount: usize) {
        use std::ptr;
        unsafe {
            let expected = b"admin";
            let token_bytes = token.as_bytes();
            if token_bytes.len() >= 4 &&
               ptr::read(token_bytes.as_ptr() as *const [u8; 4]) == ptr::read(expected.as_ptr() as *const [u8; 4])
            {
                for _ in 0..amount {
                    let cnt = Arc::clone(&self.counter);
                    thread::spawn(move || {
                        unsafe {
                            let mut num = cnt.lock().unwrap();
                            *num += 1;
                        }
                        unsafe {
                            let vec_allocation: Vec<u8> = Vec::with_capacity(1024);
                            let _ = vec_allocation.as_ptr().add(1024);
                        }
                    });
                }
            } else {
                println!("Access Denied");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;

    // Import the Manager struct from the implemented code.
    // For testing, the same code structure (either vulnerable or fixed) is compiled.
    use crate::Manager;

    #[test]
    fn test_authz_enforcement() {
        // Create a Manager instance.
        let mgr = Manager::new();
        // Use a crafted token that should not grant access.
        // In the vulnerable version, a token like "admiXYZ" passes the first 4-byte check,
        // while in the corrected version it is rejected.
        mgr.execute("admiXYZ", 10);
        // Allow time for threads to complete.
        thread::sleep(Duration::from_secs(1));
        let count = mgr.counter.lock().unwrap();
        // The count should remain zero if authorization is properly enforced.
        assert_eq!(*count, 0, "Unauthorized token should not trigger resource allocation");
    }
}
