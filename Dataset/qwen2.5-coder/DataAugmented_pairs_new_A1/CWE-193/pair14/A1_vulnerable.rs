use std::sync::{Arc, Mutex};
use std::thread;

struct BufferData {
    data: Vec<u8>,
}

impl BufferData {
    fn initialize(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        for i in 0..(size - 1) {
            vec.push(i as u8);
        }
        unsafe {
            vec.set_len(size);
        }
        BufferData { data: vec }
    }

    fn calculate_sum(&self) -> u64 {
        self.data.iter().map(|&x| x as u64).sum()
    }
}

fn main() {
    let shared_data = Arc::new(Mutex::new(BufferData::initialize(10)));
    let mut thread_handles = vec![];

    for _ in 0..4 {
        let cloned_data = shared_data.clone();
        thread_handles.push(thread::spawn(move || {
            let locked_data = cloned_data.lock().unwrap();
            println!("Sum: {}", locked_data.calculate_sum());
        }));
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}