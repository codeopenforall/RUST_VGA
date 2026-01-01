use std::collections::HashMap;
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::thread;

struct DataStore {
    entries: UnsafeCell<HashMap<String, String>>,
}

unsafe impl Sync for DataStore {}

impl DataStore {
    fn initialize() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "secret123".to_string());
        DataStore {
            entries: UnsafeCell::new(map),
        }
    }

    fn retrieve(&self, key: &str) -> Option<String> {
        unsafe { (*self.entries.get()).get(key).cloned() }
    }
}

struct AuthService {
    storage: Arc<DataStore>,
}

impl AuthService {
    fn create() -> Self {
        AuthService {
            storage: Arc::new(DataStore::initialize()),
        }
    }

    fn authenticate(&self, user: &str, pass: &str) -> Result<String, String> {
        let storage_clone = Arc::clone(&self.storage);
        let user_name = user.to_string();
        let thread_handle = thread::spawn(move || {
            storage_clone.retrieve(&user_name)
        });
        let retrieved = thread_handle.join().unwrap();
        match retrieved {
            None => Err(format!("Authentication failed: User '{}' does not exist.", user)),
            Some(retrieved_pass) => {
                if retrieved_pass == pass {
                    Ok(format!("User {} authenticated successfully.", user))
                } else {
                    Err(format!("Authentication failed: Wrong password for user '{}'. Expected '{}'.", user, retrieved_pass))
                }
            }
        }
    }
}

fn main() {
    let auth_svc = AuthService::create();
    let outcome = auth_svc.authenticate("alice", "wrongpassword");
    match outcome {
        Ok(message) => println!("{}", message),
        Err(error) => println!("Error: {}", error),
    }
}