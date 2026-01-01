use std::sync::{Arc, Mutex};
use std::thread;
pub struct DataHolder {
    data: Vec<u32>,
}
impl DataHolder {
    pub fn new(v: Vec<u32>) -> Self {
        DataHolder { data: v }
    }
    pub fn retrieve(&self, idx: usize) -> Option<u32> {
        if idx < self.data.len() {
            Some(self.data[idx])
        } else {
            None
        }
    }
}
fn main() {
    let holder = DataHolder::new(vec![10, 20, 30]);
    let shared = Arc::new(Mutex::new(holder));
    let mut handles = vec![];
    for i in 0..5 {
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let holder = shared_clone.lock().unwrap();
            let index = i * 2;
            match holder.retrieve(index) {
                Some(value) => println!("Thread {} read: {}", i, value),
                None => println!("Thread {}: index {} out-of-bound", i, index),
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
}

#[cfg(test)]
mod tests {
    use super::DataHolder;

    #[test]
    fn test_out_of_bound_read() {
        // Create a DataHolder with a vector of three elements.
        // Attempt to retrieve an element at index 4 (which is out-of-bound).
        // In the safe (fixed) version, this should return None.
        // In the vulnerable version, the unchecked memory access will likely return Some(garbage)
        // or trigger undefined behavior. Thus, this test serves as a contract.
        let holder = DataHolder::new(vec![10, 20, 30]);
        let result = holder.retrieve(4);
        assert!(result.is_none(), "Expected None for out-of-bound read, but got {:?}", result);
    }
}
