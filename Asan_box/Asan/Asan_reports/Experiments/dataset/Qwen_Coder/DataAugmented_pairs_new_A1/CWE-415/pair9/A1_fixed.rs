use std::sync::atomic::{AtomicBool, Ordering};
static FREED: AtomicBool = AtomicBool::new(false);
struct Item {
    data: *mut i32,
}
impl Item {
    fn new() -> Self {
        let boxed = Box::new(100);
        Item { data: Box::into_raw(boxed) }
    }
    fn value(&self) -> i32 {
        unsafe { *self.data }
    }
}
impl Clone for Item {
    fn clone(&self) -> Self {
        Item { data: self.data }
    }
}
impl Drop for Item {
    fn drop(&mut self) {
        unsafe {
            if !FREED.load(Ordering::SeqCst) {
                FREED.store(true, Ordering::SeqCst);
                drop(Box::from_raw(self.data));
            }
        }
    }
}
fn run_app() {
    FREED.store(false, Ordering::SeqCst);
    let first = Item::new();
    let second = first.clone();
    println!("Values: {} and {}", first.value(), second.value());
}
fn main() {
    run_app();
}

////////////////////////////////////////////////////////////////
// Test oracle function to differentiate the behavior.
// When running the vulnerable version the double release will trigger a panic,
// whereas the corrected version will execute without panicking.
#[cfg(test)]
mod oracle {
    use std::panic;

    // The run_app() function is invoked in a catch_unwind block.
    // For the vulnerable version, a panic ("Double free detected") will occur,
    // causing this test to fail. For the corrected version, no panic occurs, so the test passes.
    #[test]
    fn test_item_behavior() {
        let result = panic::catch_unwind(|| {
            // Assumes run_app() is the entry function for the application.
            crate::run_app();
        });
        assert!(result.is_ok(), "Test oracle: double release error detected.");
    }
}
