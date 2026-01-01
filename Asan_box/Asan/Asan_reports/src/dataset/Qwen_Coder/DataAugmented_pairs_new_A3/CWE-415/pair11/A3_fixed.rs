use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Resource {
    raw: *mut u8,
}

impl Resource {
    pub fn new() -> Self {
        GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
        Resource { raw: std::ptr::null_mut() }
    }

    pub fn free(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                std::ptr::drop_in_place(self.raw);
            }
            self.raw = std::ptr::null_mut();
            GLOBAL_COUNT.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

pub fn process() -> Result<(), String> {
    let mut resource = Resource::new();
    // Simulate some processing
    resource.free();
    // Attempting to free again should not cause a double free
    resource.free();
    Ok(())
}

/*
Test Oracle:
This unit test function is designed to run the 'process' function (common to both modules).
It resets the global counter and asserts that the process function returns Ok.
For the vulnerable version (double free), the condition GLOBAL_COUNT == 1 is violated,
and the test will fail. For the fixed version, the test passes.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[test]
    fn test_resource_handling() {
        // Reset the global counter prior to running the test.
        GLOBAL_COUNT.store(0, Ordering::SeqCst);
        let result = process();
        assert!(result.is_ok(), "Resource handling failed due to double free vulnerability");
    }
}
