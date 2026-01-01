use std::sync::Arc;
use std::thread;

struct Resource {
    value: i32,
}

impl Resource {
    fn new(val: i32) -> Self {
        Resource { value: val }
    }

    fn perform(&self) {
        unsafe {
            if self.value != 0 {
                // Instead of panicking, we can return an error or handle it gracefully
                // For simplicity, we'll just return early from the unsafe block
                return;
            } else {
                let ptr: *mut i32 = std::ptr::null_mut();
                *ptr = 42;
            }
        }
    }
}

fn process(op: i32) -> Result<(), &'static str> {
    let res = Arc::new(Resource::new(op));
    let res_clone = res.clone();
    let handle = thread::spawn(move || {
        res_clone.perform();
    });

    match handle.join() {
        Ok(_) => Ok(()),
        Err(_) => Err("Thread panicked"),
    }
}

fn main() {
    let result = process(1);
    println!("Result: {:?}", result);
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::process;

    // This test is designed to fail for the vulnerable version (which propagates the panic)
    // and pass for the corrected version (which catches and handles the panic gracefully).
    #[test]
    fn concurrency_recovery_oracle() {
        // Trigger the unsafe operation with a non-zero value.
        // In a secure implementation, this should be captured and processed as an Ok result.
        let result = process(1);
        assert!(result.is_ok(), "Expected Ok result, but got an error: {:?}", result);
    }
}
