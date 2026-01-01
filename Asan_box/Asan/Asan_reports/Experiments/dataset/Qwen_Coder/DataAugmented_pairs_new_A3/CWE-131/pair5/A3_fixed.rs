use std::alloc::{Layout, alloc};

struct ConcurrentWorker;

impl ConcurrentWorker {
    pub fn execute(&self, trigger: &str) -> bool {
        let len = trigger.len();
        let layout = Layout::from_size_align(len + 1, 1).unwrap();
        unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                return false;
            }
            // Simulate some work with the buffer
            // ...
            std::alloc::dealloc(ptr, layout);
        }
        true
    }
}

/////////////////////// Test Oracle ///////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        let trigger = "overflow";
        let worker = ConcurrentWorker;
        // In the vulnerable code, the miscalculation causes the function to return false,
        // whereas in the fixed code it correctly returns true.
        assert!(
            worker.execute(trigger),
            "Test oracle failed: expected success for properly allocated buffer."
        );
    }
}
