use std::vec::Vec;
use std::sync::{Arc, Mutex};

struct DataBlock {
    data: Vec<u32>,
}

impl DataBlock {
    fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        unsafe {
            vec.set_len(size);
        }
        Self { data: vec }
    }

    fn fill(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = i as u32;
        }
    }

    fn verify(&self) -> bool {
        for i in 0..self.data.len() {
            if self.data[i] != i as u32 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let shared = Arc::new(Mutex::new(DataBlock::new(10)));
    {
        let shared_clone = Arc::clone(&shared);
        let handle = std::thread::spawn(move || {
            let mut db = shared_clone.lock().unwrap();
            db.fill();
        });
        handle.join().unwrap();
    }
    let db = shared.lock().unwrap();
    println!("Buffer contents (fixed): {:?}", db.data);
}