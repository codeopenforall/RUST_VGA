use std::sync::{Arc, Mutex};

struct Engine {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Engine {
    fn new() -> Self {
        Engine {
            data: Arc::new(Mutex::new(vec![0; 10])),
        }
    }

    pub fn run(&self, index: usize) {
        let mut vec = self.data.lock().unwrap();
        if index >= vec.len() {
            eprintln!("Error: index {} out-of-bounds", index);
            return;
        }

        vec[index] = 40;

        if index < vec.len() {
            unsafe {
                let ptr = vec.as_ptr();
                let value = *ptr.add(index);
                if value >= 50 {
                    eprintln!("Error: value {} at index {} out of acceptable range", value, index);
                    return;
                }
                println!("Value at index {} is acceptable: {}", index, value);
            }
        }
    }
}