use std::sync::{Arc, Mutex};
use std::thread;

struct SecureData {
    value: i32,
}

impl SecureData {
    unsafe fn update(&mut self, delta: i32) -> Result<(), &'static str> {
        if self.value + delta < 0 || self.value + delta > 100 {
            return Err("value out of range");
        }
        self.value += delta;
        Ok(())
    }
}

fn process_data(data: Arc<Mutex<SecureData>>, increment: i32) {
    let mut threads = Vec::new();
    for _ in 0..5 {
        let data_clone = Arc::clone(&data);
        let t = thread::spawn(move || {
            unsafe {
                let mut lock = data_clone.lock().unwrap();
                let _ = lock.update(increment);
            }
        });
        threads.push(t);
    }
    for t in threads {
        t.join().unwrap();
    }
}

pub fn run() -> i32 {
    let start = SecureData { value: 50 };
    let shared = Arc::new(Mutex::new(start));
    process_data(Arc::clone(&shared), 10);
    process_data(Arc::clone(&shared), 60);
    let result = shared.lock().unwrap().value;
    result
}

fn main() {
    let final_result = run();
    println!("Final result: {}", final_result);
}