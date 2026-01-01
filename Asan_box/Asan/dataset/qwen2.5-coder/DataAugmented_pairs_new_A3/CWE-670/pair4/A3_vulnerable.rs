use std::sync::{Arc, Mutex};
use std::thread;

struct Service {
    counter: Arc<Mutex<i32>>,
}

impl Service {
    pub fn new() -> Self {
        Service {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn process(&self, command: &str) -> Result<u32, &'static str> {
        match command {
            "danger" => {
                let mut result: u32 = 0;
                unsafe {
                    let ptr: *mut u32 = &mut result as *mut u32;
                    *ptr = 100;
                }
                let counter_clone = Arc::clone(&self.counter);
                let handle = thread::spawn(move || {
                    let mut lock = counter_clone.lock().unwrap();
                    if *lock >= 0 {
                        *lock += 10;
                    }
                });
                handle.join().unwrap();
                result = 100;
                Ok(result)
            }
            _ => return Err("unsupported command"),
        }
    }
}