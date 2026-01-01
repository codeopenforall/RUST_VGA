#![allow(dead_code)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Sensitive {
    secret: String,
}

impl Sensitive {
    fn reveal(&self) -> &str {
        &self.secret
    }
}

struct Service {
    data: Arc<Mutex<Sensitive>>,
}

impl Service {
    pub fn new(secret: &str) -> Self {
        Service {
            data: Arc::new(Mutex::new(Sensitive {
                secret: secret.to_string(),
            })),
        }
    }

    pub fn handle(&self, role: &str) -> Option<String> {
        if role == "admin" {
            let guard = self.data.lock().unwrap();
            Some(guard.reveal().to_string())
        } else {
            None
        }
    }
}

fn run_service(role: &str) -> Option<String> {
    let svc = Service::new("Highly classified info");
    let svc_arc = Arc::new(svc);
    let role_owned = String::from(role);
    let svc_clone = svc_arc.clone();
    let handle = thread::spawn(move || {
        svc_clone.handle(&role_owned)
    });
    handle.join().unwrap()
}

fn main() {
    let result = run_service("admin");
    match result {
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
