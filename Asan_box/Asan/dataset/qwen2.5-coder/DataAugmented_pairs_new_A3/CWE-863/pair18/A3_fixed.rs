use std::sync::{Arc, Mutex};
use std::thread;

struct Manager {
    counter: Arc<Mutex<i32>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn execute(&self, token: &str, amount: usize) {
        if token == "admin" {
            let limited = if amount > 100 { 100 } else { amount };
            for _ in 0..limited {
                let cnt = Arc::clone(&self.counter);
                thread::spawn(move || {
                    let mut num = cnt.lock().unwrap();
                    *num += 1;
                    let _vec: Vec<u8> = Vec::with_capacity(1024);
                });
            }
        } else {
            eprintln!("Access Denied");
        }
    }
}