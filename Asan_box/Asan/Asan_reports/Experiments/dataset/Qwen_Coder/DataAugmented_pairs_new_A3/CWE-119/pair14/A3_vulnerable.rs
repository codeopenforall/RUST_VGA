use std::sync::{Arc, Mutex};
use std::thread;

struct DataStore {
    storage: Box<[u32]>,
}

impl DataStore {
    fn initialize(capacity: usize) -> Self {
        let initial_data = vec![0u32; capacity].into_boxed_slice();
        DataStore { storage: initial_data }
    }

    fn insert_value(&mut self, position: usize, new_value: u32) {
        unsafe {
            let mutable_ptr = self.storage.as_mut_ptr();
            *mutable_ptr.add(position) = new_value;
        }
    }

    fn fetch_value(&self, position: usize) -> u32 {
        unsafe {
            let immutable_ptr = self.storage.as_ptr();
            *immutable_ptr.add(position)
        }
    }
}

trait Task {
    fn compute(&self, limit: usize) -> u32;
}

impl Task for DataStore {
    fn compute(&self, limit: usize) -> u32 {
        let mut total: u32 = 0;
        unsafe {
            let data_ptr = self.storage.as_ptr();
            for index in 0..limit {
                total = total.wrapping_add(*data_ptr.add(index));
            }
        }
        total
    }
}

fn execute(input: usize) -> u32 {
    let capacity = 10;
    let datastore = Arc::new(Mutex::new(DataStore::initialize(capacity)));
    {
        let datastore_clone = Arc::clone(&datastore);
        thread::spawn(move || {
            let mut ds = datastore_clone.lock().unwrap();
            ds.insert_value(input, 42);
        })
        .join()
        .unwrap();
    }
    let ds = datastore.lock().unwrap();
    ds.compute(input + 1)
}

fn main() {
    let input = 10;
    let outcome = execute(input);
    println!("Outcome: {}", outcome);
}

#[cfg(test)]
mod tests {
    // Import the execute function from the current module.
    use super::execute;
    
    #[test]
    fn memory_safety_oracle() {
        // For a safe implementation, using a boundary value should not corrupt memory.
        // Expected output is 0 because the out-of-bound write is ignored.
        let output = execute(10);
        assert_eq!(output, 0, "Memory corruption detected: result should be 0.");
    }
}
