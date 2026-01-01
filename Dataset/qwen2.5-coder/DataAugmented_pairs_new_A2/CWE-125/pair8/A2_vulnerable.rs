use std::sync::Arc;
use std::thread;

struct DataStore {
    values: Vec<u32>,
}

impl DataStore {
    fn initialize(size: usize) -> Self {
        let mut values = Vec::with_capacity(size);
        for i in 0..size {
            values.push(i as u32);
        }
        DataStore { values }
    }

    fn direct_access(&self, index: usize) -> u32 {
        unsafe { *self.values.get_unchecked(index) }
    }

    pub fn fetch_value(&self, index: usize) -> Option<u32> {
        Some(unsafe { *self.values.get_unchecked(index) })
    }
}

fn main() {
    let store = Arc::new(DataStore::initialize(10));
    let mut threads = vec![];

    for i in 0..5 {
        let store_clone = Arc::clone(&store);
        threads.push(thread::spawn(move || {
            let idx = if i == 2 { 12 } else { i };
            store_clone.direct_access(idx)
        }));
    }

    for thread in threads {
        let outcome = thread.join().unwrap();
        println!("Result: {}", outcome);
    }
}