use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Data {
    count: usize,
}

fn init() -> Arc<Mutex<Data>> {
    Arc::new(Mutex::new(Data { count: 0 }))
}

pub fn run_process() -> usize {
    let data = init();
    let mut handles = Vec::new();

    for _ in 0..10 {
        let data_clone = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut guard = data_clone.lock().unwrap();
            if guard.count % 2 == 0 {
                thread::sleep(Duration::from_millis(10));
                guard.count += 1;
            } else {
                thread::sleep(Duration::from_millis(10));
                guard.count += 2;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_data = data.lock().unwrap();
    final_data.count
}

fn main() {
    let result = run_process();
    println!("Final count: {}", result);
}