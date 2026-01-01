use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;

struct Credentials {
    user: String,
    secret: String,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("user", &self.user)
            .field("secret", &self.secret) // Vulnerability: secret is not redacted
            .finish()
    }
}

struct Manager {
    creds: Arc<Mutex<Credentials>>,
}

impl Manager {
    fn new(user: &str, secret: &str) -> Self {
        Manager {
            creds: Arc::new(Mutex::new(Credentials {
                user: user.to_owned(),
                secret: secret.to_owned(),
            })),
        }
    }

    fn generate_error_log(&self) -> String {
        let creds = self.creds.lock().unwrap();
        format!("Error encountered for credentials: {:?}", *creds)
    }

    fn simulate_error(&self) {
        let handle = {
            let creds_arc = Arc::clone(&self.creds);
            thread::spawn(move || {
                let _ = creds_arc.lock().unwrap().secret.clone();
            })
        };
        let _ = handle.join();
    }
}

fn main() {
    let manager = Manager::new("admin", "supersecret");
    manager.simulate_error();
    let log = manager.generate_error_log();
    println!("{}", log);
}

// Test oracle function: This unit test checks that the error log does not contain the sensitive secret
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_leak() {
        let manager = Manager::new("admin", "supersecret");
        manager.simulate_error();
        let log = manager.generate_error_log();
        // The log must NOT include the sensitive secret string "supersecret".
        assert!(
            !log.contains("supersecret"),
            "Sensitive information leaked in log: {}",
            log
        );
    }
}
