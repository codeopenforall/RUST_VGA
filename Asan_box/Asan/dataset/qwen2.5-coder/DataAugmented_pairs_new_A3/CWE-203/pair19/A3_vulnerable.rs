use std::collections::HashMap;
use std::sync::Arc;
use std::cell::UnsafeCell;

struct Store {
    data: UnsafeCell<HashMap<String, String>>,
}

unsafe impl Sync for Store {}

impl Store {
    fn new() -> Arc<Self> {
        let map = HashMap::from([
            ("alice".to_string(), "password123".to_string()),
            ("bob".to_string(), "securepass".to_string()),
        ]);
        Arc::new(Store {
            data: UnsafeCell::new(map),
        })
    }

    fn authenticate(&self, username: &str, password: &str) -> Result<String, String> {
        let user = username.to_string();
        match unsafe { (*self.data.get()).get(&user).cloned() } {
            None => Err(format!("Authentication failed: User '{}' does not exist.", username)),
            Some(stored_pass) => {
                if stored_pass == password {
                    Ok(format!("User {} authenticated successfully.", username))
                } else {
                    Err(format!("Authentication failed: Wrong password for user '{}'. Expected '{}'.", username, stored_pass))
                }
            }
        }
    }
}

struct Service {
    store: Arc<Store>,
}

impl Service {
    fn new() -> Self {
        Service {
            store: Store::new(),
        }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<String, String> {
        self.store.authenticate(username, password)
    }
}