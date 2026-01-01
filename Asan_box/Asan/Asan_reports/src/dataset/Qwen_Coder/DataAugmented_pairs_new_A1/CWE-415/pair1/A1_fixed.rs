use std::sync::atomic::{AtomicBool, Ordering};
static FREED: AtomicBool = AtomicBool::new(false);
struct Data {
    ptr: *mut i32,
}
impl Data {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        Data { ptr: Box::into_raw(boxed) }
    }
}
impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            if !FREED.compare_and_swap(false, true, Ordering::SeqCst) {
                let _ = Box::from_raw(self.ptr);
            } else {
                // Avoid double free by not freeing again
                // let _ = Box::from_raw(self.ptr);
                // Instead, just do nothing
            }
        }
    }
}
fn run_app() -> i32 {
    let instance1 = Data::new(100);
    let _instance2 = Data { ptr: instance1.ptr };
    100
}
fn main() {
    let _ = run_app();
}

#[cfg(test)]
mod tests {
    use std::panic;

    // The test oracle ensures that the run_app function does NOT panic.
    // In the presence of the double free vulnerability, the vulnerable version will panic,
    // causing the test to fail. The corrected version, however, will run to completion.
    //
    // Note: The test is identical for both versions and will report a failure if a panic is caught.
    #[test]
    fn test_run_app_integrity() {
        let result = panic::catch_unwind(|| {
            // Depending on which version is compiled (vulnerable or corrected),
            // this call will either panic (vulnerable) or succeed (corrected).
            crate::run_app();
        });
        assert!(result.is_ok(), "Application panicked due to double free vulnerability");
    }
}
