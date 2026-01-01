use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    value: i32,
    flag: bool,
}

impl Data {
    fn create_init() -> Self {
        Data { value: 42, flag: false }
    }
}

fn compute() -> (i32, bool) {
    let shared = Arc::new(Mutex::new(Data { value: 0, flag: false }));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let safe_data = Data::create_init();
            let mut data = shared_clone.lock().unwrap();
            data.value = data.value.wrapping_add(safe_data.value);
            if safe_data.flag {
                data.flag = true;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_data = shared.lock().unwrap();
    (final_data.value, final_data.flag)
}

fn main() {
    let result = compute();
    println!("Result: {:?}", result);
}