use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct App {
    storage: Arc<Mutex<HashMap<String, String>>>,
}

#[derive(Debug, PartialEq)]
enum ErrorType {
    InvalidInput,
}

impl App {
    fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("alice".to_string(), "secret".to_string());
        App {
            storage: Arc::new(Mutex::new(users)),
        }
    }

    fn verify(&self, account: &str, secret: &str) -> Result<(), ErrorType> {
        let users = self.storage.lock().unwrap();
        let is_valid = if let Some(valid_secret) = users.get(account) {
            valid_secret == secret
        } else {
            false
        };
        if is_valid {
            Ok(())
        } else {
            Err(ErrorType::InvalidInput)
        }
    }
}

fn main() {
    let service = App::new();
    let service_arc = Arc::new(service);
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let srv = service_arc.clone();
            thread::spawn(move || {
                let _ = srv.verify("alice", if i % 2 == 0 { "wrong" } else { "secret" });
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

//////////////// Test Oracle Function Start
// This oracle is designed to check that both invalid scenarios yield identical error responses.
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_uniformity() {
        let service = App::new();
        // Case 1: account exists with wrong secret.
        let res_existing = service.verify("alice", "wrong");
        // Case 2: account does not exist.
        let res_missing = service.verify("bob", "anything");
        
        match (res_existing, res_missing) {
            (Err(e1), Err(e2)) => {
                // For secure code, both errors should be identical.
                assert_eq!(format!("{:?}", e1), format!("{:?}", e2),
                   "Error messages differ, leaking sensitive state.");
            },
            _ => panic!("Expected both cases to fail uniformly"),
        }
    }
}
//////////////// Test Oracle Function End
