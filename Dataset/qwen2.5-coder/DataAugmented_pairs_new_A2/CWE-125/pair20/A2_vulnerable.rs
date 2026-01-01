use std::sync::Arc;
use std::thread;

struct DataHolder {
    elements: Vec<u32>,
}

impl DataHolder {
    fn new(size: usize) -> Self {
        DataHolder {
            elements: (0..size as u32).collect(),
        }
    }

    fn fetch_element(&self, idx: usize) -> Option<u32> {
        unsafe { Some(*self.elements.get_unchecked(idx)) }
    }
}

fn main() {
    let holder = Arc::new(DataHolder::new(10));
    let shared = holder.clone();
    let handle = thread::spawn(move || {
        let outcome = shared.fetch_element(10);
        match outcome {
            Some(val) => println!("Thread fetched value: {}", val),
            None => println!("Thread encountered None"),
        }
    });
    let _ = handle.join();
}