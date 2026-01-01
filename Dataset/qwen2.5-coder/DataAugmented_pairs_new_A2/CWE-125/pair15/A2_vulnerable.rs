use std::sync::Arc;
use std::thread;

struct DataHolder {
    elements: Vec<u32>,
}

impl DataHolder {
    fn calculate(&self) -> u32 {
        unsafe {
            *self.elements.get_unchecked(self.elements.len() - 1)
        }
    }
}

fn main() {
    let data = Arc::new(DataHolder { elements: vec![1, 2, 3, 4, 5] });
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || data_clone.calculate());
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}