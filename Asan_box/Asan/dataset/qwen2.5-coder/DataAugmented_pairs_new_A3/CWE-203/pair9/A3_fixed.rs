use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct User {
    username: String,
    password: String,
}

struct AuthSystem {
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl AuthSystem {
    fn new() -> Self {
        AuthSystem {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn register(&self, username: &str, password: &str) {
        let mut users = self.users.lock().unwrap();
        users.insert(
            username.to_string(),
            User {
                username: username.to_string(),
                password: password.to_string(),
            },
        );
    }

    fn verify(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.users.lock().unwrap();
        if let Some(user) = users.get(username) {
            if user.password == password {
                Ok(())
            } else {
                Err("Authentication failed".to_string())
            }
        } else {
            Err("Authentication failed".to_string())
        }
    }
}

fn main() {
    let auth = AuthSystem::new();
    auth.register("alice", "secret");
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let auth_clone = AuthSystem {
                users: Arc::clone(&auth.users),
            };
            thread::spawn(move || {
                let _ = auth_clone.verify("alice", "secret");
            })
        })
        .collect();
    for handle in handles {
        let _ = handle.join();
    }
    match auth.verify("alice", "wrong") {
        Ok(_) => println!("Logged in successfully."),
        Err(e) => println!("Error: {}", e),
    }
}