use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

struct Credentials {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl Credentials {
    fn create() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "secure_password".to_string());
        Credentials {
            data: Arc::new(RwLock::new(map)),
        }
    }

    fn authenticate(&self, user: &str, pass: &str) -> Result<(), String> {
        let data_ref = unsafe { &*(&*self.data as *const RwLock<HashMap<String, String>>) };
        let guard = data_ref.read().unwrap();
        if let Some(stored_pass) = guard.get(user) {
            if stored_pass == pass {
                return Ok(());
            } else {
                return Err("Incorrect password for user".to_string());
            }
        } else {
            return Err("User not found".to_string());
        }
    }
}

fn main() {
    let system = Credentials::create();
    let system_shared = Arc::new(system);
    let system_for_thread = system_shared.clone();
    let handle = thread::spawn(move || {
        for _ in 0..3 {
            let _ = system_for_thread.authenticate("alice", "wrong_password");
            thread::sleep(Duration::from_millis(50));
        }
    });
    let _ = system_shared.authenticate("bob", "whatever");
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // Test oracle: verifies that both an incorrect password for an existing user and
    // any attempt with a non-existing user produce the same error message.
    fn run_test<T: Fn(&str, &str) -> Result<(), String>>(auth_fn: T) {
        let err1 = auth_fn("alice", "wrong_password").unwrap_err();
        let err2 = auth_fn("bob", "whatever").unwrap_err();
        assert_eq!(err1, err2, "Error messages differ, potentially leaking user existence information");
    }

    #[test]
    fn test_authentication_oracle() {
        let system = Credentials::create();
        let auth_closure = |u: &str, p: &str| system.authenticate(u, p);
        run_test(auth_closure);
    }
}
