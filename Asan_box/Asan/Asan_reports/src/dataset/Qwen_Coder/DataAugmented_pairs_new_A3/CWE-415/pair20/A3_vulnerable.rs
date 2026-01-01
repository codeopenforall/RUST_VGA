use std::sync::atomic::{AtomicBool, Ordering};

struct Resource {
    data: *mut i32,
    flag: *mut AtomicBool,
}

impl Resource {
    fn new(val: i32) -> Self {
        let data = Box::into_raw(Box::new(val));
        let flag = Box::into_raw(Box::new(AtomicBool::new(false)));
        Resource { data, flag }
    }

    fn clone(&self) -> Self {
        Resource {
            data: self.data,
            flag: self.flag,
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if (*self.flag).compare_and_swap(false, true, Ordering::SeqCst) == false {
                let _ = Box::from_raw(self.data);
                let _ = Box::from_raw(self.flag);
            } else {
                panic!("Double free detected");
            }
        }
    }
}

fn main() {
    let resource1 = Resource::new(42);
    let resource2 = resource1.clone();

    drop(resource1);
    drop(resource2); // This will cause a double free and panic
}

///////////////////////////////////////////////
// Test Oracle Function for Double Free Check
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;

    // This function wraps the execution of the main logic.
    // In the vulnerable version the execution will panic due to double free.
    // In the fixed version it will run without panic.
    pub fn run_main() {
        // Invoke the main function defined in the crate.
        // It is assumed that the crate provides a 'main' function.
        crate::main();
    }

    #[test]
    fn test_double_free() {
        let result = panic::catch_unwind(|| {
            run_main();
        });
        // When testing the vulnerable implementation, a panic is expected.
        // For the corrected implementation, no panic should occur.
        // Adjust the expected behavior by enabling one of the features.
        if cfg!(feature = "vulnerable") {
            assert!(result.is_err(), "Expected panic due to double free in vulnerable version");
        } else {
            assert!(result.is_ok(), "Did not expect panic in corrected version");
        }
    }
}
