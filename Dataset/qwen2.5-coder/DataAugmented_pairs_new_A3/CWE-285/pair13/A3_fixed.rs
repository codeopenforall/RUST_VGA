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