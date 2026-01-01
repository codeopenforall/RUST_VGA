use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Record {
    username: String,
    password: String, 
}

struct Database {
    users: Arc<Mutex<HashMap<String, Record>>>,
}

impl Database {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(
            "alice".to_string(),
            Record {
                username: "alice".into(),
                password: "secret123".into(),
            },
        );
        Database {
            users: Arc::new(Mutex::new(map)),
        }
    }

    unsafe fn search(&self, uname: &str) -> Option<*const Record> {
        let guard = self.users.lock().unwrap();
        if let Some(user) = guard.get(uname) {
            Some(user as *const Record)
        } else {
            None
        }
    }
}

fn verify(db: &Database, username: &str, password: &str) -> Result<(), String> {
    unsafe {
        if let Some(user_ptr) = db.search(username) {
            let user = &*user_ptr;
            if user.password == password {
                Ok(())
            } else {
                Err("Invalid credentials".to_string())
            }
        } else {
            Err("Invalid credentials".to_string())
        }
    }
}

fn main() {
    let db = Database::new();
    let db_arc = Arc::new(db);
    let db_clone = db_arc.clone();
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        match verify(&db_clone, "alice", "wrongpass") {
            Ok(_) => println!("Login succeeded"),
            Err(e) => println!("Login failed: {}", e),
        }
    });
    handle.join().unwrap();
}

///////////////////////////////////////////////////////////////
// Test Oracle Function
// This test oracle verifies that authentication failures consistently return
// the unified error message ("Invalid credentials") regardless of whether the
// username exists or the password is incorrect. For the vulnerable version, this
// test will fail due to inconsistent error messages.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the appropriate items from the module containing the implementation.
    use super::*;

    #[test]
    fn test_authentication_error_message() {
        let db = Database::new();

        // Test with a valid username but incorrect password.
        let err_msg1 = verify(&db, "alice", "wrongpass").err().unwrap();
        // Test with an invalid username.
        let err_msg2 = verify(&db, "bob", "any_pass").err().unwrap();

        // Both calls should produce the same error message.
        assert_eq!(err_msg1, "Invalid credentials", "Unexpected error message for invalid password");
        assert_eq!(err_msg2, "Invalid credentials", "Unexpected error message for non-existent user");
    }
}
