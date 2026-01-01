use std::sync::{Arc, Mutex};
use std::thread;

struct DataPool {
    buffer: Vec<u8>,
}

trait Modifier {
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str>;
}

impl Modifier for DataPool {
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index >= self.buffer.len() {
            return Err("Index out of bounds");
        }
        self.buffer[index] = value;
        Ok(())
    }
}

impl DataPool {
    fn new(size: usize) -> Self {
        DataPool {
            buffer: vec![0; size],
        }
    }
}

fn main() {
    let pool = Arc::new(Mutex::new(DataPool::new(10)));
    let pool_clone = Arc::clone(&pool);
    let handle = thread::spawn(move || {
        let mut dp = pool_clone.lock().unwrap();
        dp.modify(5, 42).unwrap();
    });
    handle.join().unwrap();
    let mut dp = pool.lock().unwrap();
    dp.modify(11, 99).unwrap();
    println!("Final buffer: {:?}", dp.buffer);
}