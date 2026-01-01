#![allow(dead_code)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Confidential {
    secret: String,
}

impl Confidential {
    unsafe fn expose(&self) -> &str {
        &*(&self.secret as *const String)
    }
}

struct System {
    info: Arc<Mutex<Confidential>>,
}

impl System {
    pub fn initialize(secret: &str) -> Self {
        System {
            info: Arc::new(Mutex::new(Confidential {
                secret: secret.to_string(),
            })),
        }
    }

    pub fn process(&self, position: &str) -> Option<String> {
        if position == "admin" || position == "user" {
            let lock = self.info.lock().unwrap();
            unsafe {
                Some(lock.expose().to_string())
            }
        } else {
            None
        }
    }
}

fn execute_system(position: &str) -> Option<String> {
    let sys = System::initialize("Confidential information");
    let sys_arc = Arc::new(sys);
    let position_owned = String::from(position);
    let sys_clone = sys_arc.clone();
    let task = thread::spawn(move || {
        sys_clone.process(&position_owned)
    });
    task.join().unwrap()
}

fn main() {
    let outcome = execute_system("user");
    match outcome {
        Some(data) => println!("Data: {}", data),
        None => println!("Access Denied"),
    }
}