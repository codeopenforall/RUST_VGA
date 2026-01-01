use std::sync::Arc;

pub struct DataHolder {
    pub data: Vec<u32>,
}

impl DataHolder {
    pub fn new(size: usize) -> Self {
        DataHolder { data: vec![0; size] }
    }

    pub unsafe fn get_ptr(&self) -> *const u32 {
        self.data.as_ptr()
    }
}

pub fn critical_run() {
    let holder = Arc::new(DataHolder::new(10));
    let holder_clone = Arc::clone(&holder);

    unsafe {
        let ptr = holder_clone.data.as_ptr() as *mut u32;
        *ptr.add(5) = 42;
    }

    unsafe {
        let ptr = holder.get_ptr();
        let value = *ptr.add(5);
        assert!(value != 42, "Invariant violated: unexpected trigger value encountered.");
    }
}

///////////////////////////////////////////////
// Test Oracle: This unit test is applied externally.
// It calls the public critical_run() function and asserts that it completes normally.
// For the vulnerable implementation, this test will catch a panic and fail.
// For the corrected implementation, critical_run() will execute without panicking.
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test executes the critical function and expects no panic.
    #[test]
    fn test_critical_run() {
        let result = panic::catch_unwind(|| {
            critical_run();
        });
        // The invariant violation should not cause a panic in the corrected code.
        assert!(result.is_ok(), "critical_run() panicked unexpectedly");
    }
}
