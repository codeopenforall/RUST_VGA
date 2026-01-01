use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    secret: i32,
}

struct Manager {
    data: Arc<Mutex<Data>>,
}

impl Manager {
    fn new(secret: i32) -> Self {
        Self {
            data: Arc::new(Mutex::new(Data { secret })),
        }
    }

    fn do_action(&self) -> i32 {
        let ptr: *const i32;
        unsafe {
            let data_ref = self.data.lock().unwrap();
            ptr = &data_ref.secret as *const i32;
        }
        unsafe { *ptr }
    }

    fn run(&self, is_admin: bool) -> Result<i32, &'static str> {
        if !is_admin {
            return Err("Unauthorized access");
        }
        Ok(self.do_action())
    }
}

fn main() {
    let manager = Manager::new(42);
    let m = Arc::new(manager);
    let handles: Vec<_> = (0..5).map(|_| {
        let m_clone = m.clone();
        thread::spawn(move || {
            m_clone.run(false)
        })
    }).collect();
    for handle in handles {
        let result = handle.join().unwrap();
        match result {
            Ok(val) => println!("Result is: {}", val),
            Err(err) => println!("Error: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    // To run these tests, include the Manager definition from the target code.
    use super::*;
    
    #[test]
    fn test_unauthorized_access() {
        // Create a Manager with a known secret.
        let manager = Manager::new(42);
        // Simulate an unauthorized call.
        let result = manager.run(false);
        // In the corrected version, unauthorized access must be rejected.
        assert!(result.is_err(), "Unauthorized access should not be allowed");
        if let Err(e) = result {
            assert_eq!(e, "Unauthorized access", "Unexpected error message");
        }
    }
}
