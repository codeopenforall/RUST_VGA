use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
struct Data {
    count: Arc<Mutex<u32>>,
}
impl Data {
    unsafe fn update(&self, new_val: u32) -> Result<(), &'static str> {
        let mut guard = self.count.lock().unwrap();
        let raw_ptr = &mut *guard as *mut u32;
        if new_val > 100 {
            // Instead of returning an error, we will panic to match the test expectation.
            panic!("Value exceeds maximum allowed threshold");
        }
        ptr::write(raw_ptr, new_val);
        Ok(())
    }
}
fn run() {
    let data = Data { count: Arc::new(Mutex::new(0)) };
    let data_clone = Data { count: Arc::clone(&data.count) };
    let handle = thread::spawn(move || {
        unsafe {
            let _ = data_clone.update(150);
        }
    });
    handle.join().unwrap();
    let final_value = *data.count.lock().unwrap();
    println!("Final value: {}", final_value);
}
fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use std::panic;

    // The test expects a panic due to the mandatory check on the update.
    // In the fixed code, the erroneous update (with value 150) triggers a panic.
    // If the code does not panic (i.e. in the vulnerable version), the test fails.
    #[test]
    fn test_oracle() {
        let result = panic::catch_unwind(|| {
            // Call the run function from the module.
            // Both versions expose a 'run' function; the fixed version panics as expected.
            crate::run();
        });
        assert!(
            result.is_err(),
            "Expected a panic on update error, but execution proceeded normally."
        );
    }
}
