use std::sync::{Arc, Mutex};

struct Holder {
    data: Vec<i32>,
}

impl Holder {
    fn new(size: usize) -> Self {
        Holder {
            data: vec![0; size],
        }
    }

    fn update(&mut self, index: usize, value: i32) {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_mut_ptr();
                *ptr.add(index) = value;
            }
        } else {
            panic!("Index out of bounds: {} for length {}", index, self.data.len());
        }
    }
}

fn operate(holder: Arc<Mutex<Holder>>, index: usize, value: i32) {
    let mut holder = holder.lock().unwrap();
    holder.update(index, value);
}