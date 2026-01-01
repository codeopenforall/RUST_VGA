use std::sync::Arc;
use std::thread;

struct DataStore {
    values: Vec<u8>,
}

impl DataStore {
    fn create() -> Self {
        DataStore { values: vec![1, 2, 3, 4, 5] }
    }
    pub fn retrieve(&self, index: usize) -> u8 {
        unsafe { *self.values.get_unchecked(index) }
    }
}

pub fn execute() -> u8 {
    let store = Arc::new(DataStore::create());
    let cloned_store = Arc::clone(&store);
    let thread_handle = thread::spawn(move || {
        cloned_store.retrieve(10)
    });
    thread_handle.join().unwrap()
}

fn main() {
    let result = execute();
    println!("Result: {}", result);
}