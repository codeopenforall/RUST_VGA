use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;

struct Creds {
    user: String,
    pass: String,
}

impl fmt::Debug for Creds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Creds {{ user: {:?}, pass: \"REDACTED\" }}", self.user)
    }
}

struct Service {
    creds: Arc<Mutex<Creds>>,
}

impl Service {
    fn new(user: &str, pass: &str) -> Self {
        Self {
            creds: Arc::new(Mutex::new(Creds {
                user: user.to_owned(),
                pass: pass.to_owned(),
            })),
        }
    }

    fn process(&self) -> Result<(), String> {
        let creds = self.creds.lock().unwrap();
        let safe_info = format!("Processed request for user: {:?}", creds.user);
        Err(safe_info)
    }
}

fn main() {
    let svc = Service::new("admin", "SuperSecret!");
    let svc_clone = svc.creds.clone();
    let handle = thread::spawn(move || {
        let service = Service { creds: svc_clone };
        let res = service.process();
        if res.is_err() {
            println!("Operation encountered an error: {}", res.unwrap_err());
        }
    });
    let _ = handle.join();
    println!("Application safely exiting");
}

/*
Test Oracle: This test checks that the process method does not leak the secret.
For the vulnerable implementation the test will fail (as it leaks "SuperSecret!").
For the fixed implementation it will pass.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_leak() {
        let svc = Service::new("admin", "SuperSecret!");
        let result = svc.process();
        match result {
            Err(msg) => {
                // The fixed version should not include the secret "SuperSecret!"
                assert!(!msg.contains("SuperSecret!"), "Sensitive information leaked in message: {}", msg);
            },
            Ok(_) => panic!("Expected an error result, but got Ok."),
        }
    }
}
