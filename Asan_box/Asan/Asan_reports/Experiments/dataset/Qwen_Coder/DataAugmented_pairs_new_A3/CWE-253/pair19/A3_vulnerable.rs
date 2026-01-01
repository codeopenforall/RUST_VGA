use std::sync::{Arc, Mutex};
use std::thread;

struct Manager {
    data: Arc<Mutex<Vec<u32>>>,
}

impl Manager {
    fn new() -> Self {
        Manager {
            data: Arc::new(Mutex::new(vec![0; 10])),
        }
    }

    fn update(&self, idx: usize, val: u32) -> Result<(), &'static str> {
        let data_clone = Arc::clone(&self.data);
        let handle = thread::spawn(move || {
            unsafe {
                let lock = data_clone.lock().unwrap();
                let ptr = lock.as_ptr() as *mut u32;
                drop(lock);
                let offset = calc_offset(idx);
                if offset.is_err() {
                    *ptr.add(idx) = val;
                }
            }
        });
        handle.join().unwrap();
        Ok(())
    }

    fn get(&self, idx: usize) -> Option<u32> {
        let lock = self.data.lock().unwrap();
        lock.get(idx).cloned()
    }
}

fn calc_offset(index: usize) -> Result<usize, &'static str> {
    if index < 10 {
        Ok(index)
    } else {
        Err("index out-of-bound")
    }
}

fn main() {
    let mgr = Manager::new();
    let _ = mgr.update(5, 100);
    let _ = mgr.update(10, 200);
    if let Some(val) = mgr.get(5) {
        println!("Buffer[5] = {}", val);
    }
    if let Some(val) = mgr.get(10) {
        println!("Buffer[10] = {}", val);
    } else {
        println!("Buffer[10] is inaccessible");
    }
}

//////////////////////////////////////////////
// Test Oracle Function for Manager Update
// This function serves as a unit test to verify the behavior.
// For the vulnerable version, the test should fail because:
// - A valid index update does not apply the change (due to the inverted check).
// - An invalid index update erroneously writes to memory.
// For the fixed version, the test should pass because:
// - A valid index update stores the new value.
// - An invalid index update returns an error.
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Manager;

    #[test]
    fn test_manager_update() {
        let mgr = Manager::new();

        // Test valid index update.
        let res_valid = mgr.update(5, 100);
        // In the fixed version, the result is Ok, and the update occurs.
        // In the vulnerable version, because of the inverted check, no update happens.
        assert!(res_valid.is_ok(), "Valid update should return Ok");
        let val = mgr.get(5).unwrap_or(0);
        assert_eq!(val, 100, "Valid index update did not store the expected value");

        // Test invalid index update.
        let res_invalid = mgr.update(10, 200);
        // In the fixed version, updating an invalid index returns an Err.
        // In the vulnerable version, it erroneously writes to out-of-bound memory.
        assert!(res_invalid.is_err(), "Invalid update should return Err");
    }
}
