use std::sync::{Arc, Mutex};
use std::thread;

struct SecretData {
    value: i32,
}

struct AccessManager {
    protected_data: Arc<Mutex<SecretData>>,
}

impl AccessManager {
    fn new(secret: i32) -> Self {
        Self {
            protected_data: Arc::new(Mutex::new(SecretData { value: secret })),
        }
    }

    fn perform_action(&self) -> i32 {
        let raw_ptr: *const i32;
        unsafe {
            let data_lock = self.protected_data.lock().unwrap();
            raw_ptr = &data_lock.value as *const i32;
        }
        unsafe { *raw_ptr }
    }

    fn execute(&self, is_authorized: bool) -> Result<i32, &'static str> {
        if is_authorized {
            Ok(self.perform_action())
        } else {
            Err("Unauthorized access")
        }
    }
}

fn main() {
    let access_manager = AccessManager::new(42);
    let shared_manager = Arc::new(access_manager);
    let threads: Vec<_> = (0..5).map(|_| {
        let manager_clone = shared_manager.clone();
        thread::spawn(move || {
            manager_clone.execute(false)
        })
    }).collect();

    for thread_handle in threads {
        match thread_handle.join().unwrap() {
            Ok(value) => println!("Accessed value: {}", value),
            Err(error) => println!("Failed with error: {}", error),
        }
    }
}