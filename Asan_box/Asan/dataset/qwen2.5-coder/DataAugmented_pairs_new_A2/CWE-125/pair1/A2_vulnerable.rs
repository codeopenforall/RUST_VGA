use std::sync::Arc;
use std::thread;

struct DataStore {
    values: Vec<u32>,
}

impl DataStore {
    fn fetch(&self, idx: usize) -> u32 {
        unsafe {
            let ptr = self.values.as_ptr().offset(idx as isize - 1);
            *ptr
        }
    }
}

fn main() {
    let shared_data = Arc::new(DataStore { values: vec![40, 50, 60] });
    let shared_copy = Arc::clone(&shared_data);
    let thread_handle = thread::spawn(move || {
        let outcome = shared_copy.fetch(0);
        println!("Outcome: {}", outcome);
    });
    thread_handle.join().unwrap();
}