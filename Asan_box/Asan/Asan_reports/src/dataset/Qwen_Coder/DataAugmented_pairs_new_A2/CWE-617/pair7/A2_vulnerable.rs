use std::sync::Arc;
use std::thread;
use std::time::Duration;

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

    pub fn process_input(&self, idx: usize) {
        unsafe {
            let ptr = self.get_ptr();
            let value = *ptr.add(idx);
            assert!(value != 42, "Invariant violated: unexpected trigger value encountered.");
        }
    }
}

pub fn critical_run() {
    let holder = Arc::new(DataHolder::new(10));
    let holder_clone = Arc::clone(&holder);
    let modify_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        unsafe {
            let ptr = holder_clone.data.as_ptr() as *mut u32;
            *ptr.add(5) = 42;
        }
    });
    thread::sleep(Duration::from_millis(100));
    holder.process_input(5);
    modify_handle.join().unwrap();
}

fn main() {
    critical_run();
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
