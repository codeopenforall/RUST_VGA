use std::sync::{Arc, Mutex};
use std::thread;

const SAFE_LIMIT: usize = 100_000; // Maximum allowed allocation

struct Allocator {
    data: Arc<Mutex<Vec<u8>>>,
}

impl Allocator {
    fn new() -> Self {
        Allocator {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn allocate_with_limit(&self, n: usize) -> Result<(), &'static str> {
        if n > SAFE_LIMIT {
            return Err("Exceeded safe allocation limit");
        }

        let data_arc = self.data.clone();
        thread::spawn(move || {
            let mut vec = data_arc.lock().unwrap();
            let len = vec.len();
            vec.resize(len + n, 1u8);
        }).join().unwrap();
        Ok(())
    }
}

fn main() {
    let alloc = Allocator::new();
    match alloc.allocate_with_limit(50_000) {
        Ok(_) => println!("Allocation completed"),
        Err(e) => println!("Allocation error: {}", e),
    }
}