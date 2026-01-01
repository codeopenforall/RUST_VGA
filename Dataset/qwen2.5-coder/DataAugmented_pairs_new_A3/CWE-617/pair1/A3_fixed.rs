use std::sync::{Arc, Mutex};
use std::thread;
const THRESHOLD: usize = 100;
struct Service {
    value: usize,
}
impl Service {
    fn new() -> Self {
        Service { value: 1 }
    }
    fn update(&mut self, multiplier: usize) {
        let new_value = self.value.wrapping_mul(multiplier);
        if new_value >= THRESHOLD {
            eprintln!("Error: value exceeded safe threshold!");
        } else {
            self.value = new_value;
        }
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(Service::new()));
    let mut threads = vec![];
    for i in 1..=5 {
        let service_clone = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            let mut instance = service_clone.lock().unwrap();
            let factor = if i == 3 { 150 } else { 2 };
            instance.update(factor);
        }));
    }
    for handle in threads {
        let _ = handle.join();
    }
    println!("Final value: {}", shared.lock().unwrap().value);
}