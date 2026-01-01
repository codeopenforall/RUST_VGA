use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            data: Arc::new(Mutex::new(vec![0; 10])),
        }
    }

    pub fn run(&self, index: usize) {
        let data_clone = Arc::clone(&self.data);
        let handle = thread::spawn(move || {
            let mut vec = data_clone.lock().unwrap();
            vec[index] = 100;
            unsafe {
                let ptr = vec.as_ptr();
                let value = *ptr.add(index);
                assert!(value < 50, "Assertion triggered by unexpected state");
            }
        });

        handle.join().unwrap();
    }
}