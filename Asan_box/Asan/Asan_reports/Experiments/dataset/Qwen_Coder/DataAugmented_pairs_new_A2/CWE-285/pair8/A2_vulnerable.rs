#![allow(dead_code)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Confidential {
    secret: String,
}

impl Confidential {
    unsafe fn expose(&self) -> &str {
        &*(&self.secret as *const String)
    }
}

struct System {
    info: Arc<Mutex<Confidential>>,
}

impl System {
    pub fn initialize(secret: &str) -> Self {
        System {
            info: Arc::new(Mutex::new(Confidential {
                secret: secret.to_string(),
            })),
        }
    }

    pub fn process(&self, position: &str) -> Option<String> {
        if position == "admin" || position == "user" {
            let lock = self.info.lock().unwrap();
            unsafe {
                Some(lock.expose().to_string())
            }
        } else {
            None
        }
    }
}

fn execute_system(position: &str) -> Option<String> {
    let sys = System::initialize("Confidential information");
    let sys_arc = Arc::new(sys);
    let position_owned = String::from(position);
    let sys_clone = sys_arc.clone();
    let task = thread::spawn(move || {
        sys_clone.process(&position_owned)
    });
    task.join().unwrap()
}

fn main() {
    let outcome = execute_system("user");
    match outcome {
        Some(data) => println!("Data: {}", data),
        None => println!("Access Denied"),
    }
}

/*
Below is a test oracle function intended for unit testing. It assumes that the core logic of the service (i.e. the 
run_service function) is exposed as part of a library API in both builds (vulnerable and fixed). This single test 
asserts that invoking the service with a "user" role should return no sensitive data.

The test will fail for the vulnerable implementation (which returns Some data for a "user") and pass for the corrected
version (which returns None for a "user").
*/
#[cfg(test)]
mod tests {
    // Assumes run_service is available as part of the crate's public API.
    use crate::run_service;

    #[test]
    fn test_authorization() {
        // "user" role should not receive sensitive data.
        let result = run_service("user");
        assert!(
            result.is_none(),
            "Authorization bypass vulnerability detected: non-admin user gained access."
        );
    }
}
