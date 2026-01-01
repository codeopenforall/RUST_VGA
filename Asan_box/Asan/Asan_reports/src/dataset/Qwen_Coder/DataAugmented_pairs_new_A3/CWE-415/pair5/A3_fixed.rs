use std::sync::atomic::{AtomicBool, Ordering};
use std::panic;
static FREED: AtomicBool = AtomicBool::new(false);
struct Data {
    ptr: *mut i32,
}
impl Data {
    fn new(val: i32) -> Self {
        let b = Box::new(val);
        Self { ptr: Box::into_raw(b) }
    }
}
impl Clone for Data {
    fn clone(&self) -> Self {
        Data { ptr: self.ptr }
    }
}
impl Drop for Data {
    fn drop(&mut self) {
        if FREED.load(Ordering::SeqCst) {
            return;
        }
        FREED.store(true, Ordering::SeqCst);
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}
fn execute() {
    let original = Data::new(42);
    let duplicate = original.clone();
}
fn main() {
    let result = panic::catch_unwind(|| {
        execute();
    });
    if result.is_err() {
        eprintln!("Error encountered during execution.");
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    // This test calls the execute function and asserts that no panic occurs.
    // In the vulnerable version the double free panic will be triggered causing the test to fail.
    // In the corrected version the function completes successfully and the test passes.
    #[test]
    fn test_resource_safety() {
        let result = panic::catch_unwind(|| {
            crate::execute();
        });
        assert!(
            result.is_ok(),
            "Test failed: execution panicked due to a double free vulnerability"
        );
    }
}
