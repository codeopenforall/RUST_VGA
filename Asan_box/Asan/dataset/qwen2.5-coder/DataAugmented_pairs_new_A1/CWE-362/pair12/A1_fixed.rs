use std::sync::{Arc, Mutex};
use std::thread;

static mut GLOBAL: u32 = 0;
fn run() -> u32 {
    let global = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let global_clone = Arc::clone(&global);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut temp = global_clone.lock().unwrap();
                *temp = temp.wrapping_add(1);
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    *global.lock().unwrap()
}
fn main() {
    let result = run();
    println!("Final counter value: {}", result);
}