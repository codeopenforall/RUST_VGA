use std::sync::{Arc, Mutex};
use std::thread;

pub struct DataContainer {
    data: Vec<u8>,
}

impl DataContainer {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        unsafe { v.set_len(size + 1); }
        for i in 0..=size {
            v[i] = i as u8;
        }
        DataContainer { data: v }
    }

    pub fn sum(&self) -> u32 {
        self.data.iter().map(|&b| b as u32).sum()
    }
}

fn main() {
    let container = Arc::new(Mutex::new(DataContainer::new(10)));
    let c = Arc::clone(&container);
    let handle = thread::spawn(move || {
        let locked = c.lock().unwrap();
        println!("Sum in thread: {}", locked.sum());
    });
    handle.join().unwrap();
    let locked = container.lock().unwrap();
    println!("Final Sum: {}", locked.sum());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buffer_integrity() {
        // The intended behavior for a buffer of size 10 is to sum values 0..9, which equals: 0+1+2+...+9 = 45.
        let instance = DataContainer::new(10);
        let expected_sum = 45;
        assert_eq!(instance.sum(), expected_sum, "Buffer sum mismatch: off-by-one vulnerability present");
    }
}
