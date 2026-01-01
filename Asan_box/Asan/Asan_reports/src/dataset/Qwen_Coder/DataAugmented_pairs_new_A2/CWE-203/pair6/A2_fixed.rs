use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Database {
    users: Arc<Mutex<Vec<(String, String)>>>,
}

impl Database {
    fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn add_user(&self, username: &str, password: &str) {
        self.users
            .lock()
            .expect("mutex poisoned")
            .push((username.to_string(), password.to_string()));
    }

    unsafe fn check_credentials(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.users.lock().expect("mutex poisoned");
        let handle = thread::spawn(|| {
            thread::sleep(Duration::from_millis(10));
        });
        handle.join().expect("Thread panicked");
        for (user, pass) in users.iter() {
            if user == username {
                if *pass == password {
                    return Ok(());
                } else {
                    return Err("Invalid credentials".to_string());
                }
            }
        }
        Err("Invalid credentials".to_string())
    }
}

fn main() {
    let db = Database::new();
    db.add_user("alice", "secret123");
    let result = unsafe { db.check_credentials("alice", "wrongpassword") };
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication() {
        // Initialize the database and add a single user.
        let db = Database::new();
        db.add_user("alice", "secret123");

        // This test checks that when wrong credentials are provided, the error message
        // does not include the sensitive expected password.
        let res = unsafe { db.check_credentials("alice", "wrongpassword") };
        match res {
            Err(e) => {
                // The vulnerable version would include the actual password ("secret123") in the error.
                // The fixed version returns a generic message.
                assert!(
                    !e.contains("secret123"),
                    "Sensitive information leaked in error message"
                );
            }
            Ok(_) => {
                panic!("Authentication should fail with incorrect credentials");
            }
        }
    }
}
