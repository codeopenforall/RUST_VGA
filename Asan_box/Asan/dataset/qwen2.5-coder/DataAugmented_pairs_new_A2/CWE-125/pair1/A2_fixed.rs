use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    fn read_value(&self, index: usize) -> u32 {
        if index == 0 {
            0
        } else if index > 0 && index <= self.data.len() {
            self.data[index - 1]
        } else {
            0
        }
    }
}

fn main() {
    let shared = Arc::new(Buffer { data: vec![10, 20, 30] });
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let result = shared_clone.read_value(0);
        println!("Result: {}", result);
    });
    handle.join().unwrap();
}