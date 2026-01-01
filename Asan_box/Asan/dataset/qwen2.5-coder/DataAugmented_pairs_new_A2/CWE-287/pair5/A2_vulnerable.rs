use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

struct Session {
    token: String,
}

impl Session {
    fn new(token: &str) -> Self {
        Session { token: token.to_owned() }
    }
}

struct AuthManager {
    expected: String,
    sessions: Arc<Mutex<HashMap<u64, Session>>>,
}

impl AuthManager {
    fn new(expected: &str) -> Self {
        AuthManager {
            expected: expected.to_owned(),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn validate(&self, user_token: &str) -> bool {
        let exp_bytes = self.expected.as_bytes();
        let user_bytes = user_token.as_bytes();
        let min_len = exp_bytes.len().min(user_bytes.len());

        for i in 0..min_len {
            if exp_bytes[i] != user_bytes[i] {
                return false;
            }
        }

        // CWE-7/pair5: Introduce a defect by not checking the length of the tokens
        true
    }

    fn add_session(&self, user_id: u64, token: &str) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(user_id, Session::new(token));
    }
}

fn main() {
    let manager = AuthManager::new("SuperSecret");
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let sessions_clone = manager.sessions.clone();
            let expected_clone = manager.expected.clone();
            thread::spawn(move || {
                let auth = AuthManager::new(&expected_clone);
                let token = if i % 2 == 0 {
                    "SuperSecretExtra" 
                } else {
                    "WrongToken"
                };
                let result = auth.validate(token);
                if result {
                    let mut sessions = sessions_clone.lock().unwrap();
                    sessions.insert(i, Session::new(token));
                }
                println!("Thread {} authentication result: {}", i, result);
                result
            })
        })
        .collect();
    for handle in handles {
        let _ = handle.join().unwrap();
    }
}