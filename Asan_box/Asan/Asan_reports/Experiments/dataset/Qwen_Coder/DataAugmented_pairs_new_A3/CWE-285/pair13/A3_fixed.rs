use std::sync::{Arc, Mutex};
use std::thread;

struct User {
    name: String,
    is_admin: bool,
}

struct SecureStorage {
    data: Arc<Mutex<[u8; 16]>>,
}

impl SecureStorage {
    fn new() -> Self {
        SecureStorage { data: Arc::new(Mutex::new([0; 16])) }
    }

    fn update(&self, value: u8, user: &User) {
        if !user.is_admin {
            return; // Only allow admin users to update the data
        }
        let data_arc = self.data.clone();
        let handle = thread::spawn(move || {
            let mut data = data_arc.lock().unwrap();
            data[0] = value; // Correctly update the first element
        });
        handle.join().unwrap();
    }

    fn read(&self) -> u8 {
        let data = self.data.lock().unwrap();
        data[0]
    }
}

fn main() {
    let storage = SecureStorage::new();
    let user = User { name: "bob".to_string(), is_admin: true }; // Ensure the user is an admin
    storage.update(42, &user);
    println!("Secret: {}", storage.read());
}

//////////////////////////////////////////////
// Test oracle function to verify access control.
// This test calls the update method with a non-admin user and asserts that
// the secret remains unchanged (value 0). In the vulnerable version, the secret
// would be updated incorrectly, causing the test to fail.
// The oracle is meant to be compiled and run separately from the main examples.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_control() {
        let storage = SecureStorage::new();
        let non_admin = User { name: "eve".to_string(), is_admin: false };
        // Attempt to update the secret with a non-admin user.
        // In the fixed version, the update should be rejected.
        // In the vulnerable version, the update erroneously occurs.
        // We ignore any return value so that the test compiles for both versions.
        let _ = storage.update(123, &non_admin);
        let secret = storage.read();
        // Assert that the secret remains unchanged (remains at its default value zero).
        assert_eq!(secret, 0, "Non-admin user should not be permitted to update secret");
    }
}
