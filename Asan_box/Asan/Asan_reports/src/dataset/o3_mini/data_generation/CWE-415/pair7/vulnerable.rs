use std::sync::atomic::{AtomicBool, Ordering};
static FREED: AtomicBool = AtomicBool::new(false);
struct Resource {
    value: i32,
}
impl Drop for Resource {
    fn drop(&mut self) {
        if FREED.swap(true, Ordering::SeqCst) {
            panic!("Double free detected in drop!");
        }
    }
}
fn process() {
    let boxed = Box::new(Resource { value: 42 });
    let raw_ptr = Box::into_raw(boxed);
    unsafe {
        let first = Box::from_raw(raw_ptr);
        drop(first);
        let second = Box::from_raw(raw_ptr);
        drop(second);
    }
}
fn main() {
    process();
    println!("Completed vulnerable main function");
}


#[cfg(test)]
mod tests {
    use std::panic;

    #[test]
    fn test_memory_management() {
        // The function "process" is expected to be memory safe.
        // In the flawed (vulnerable) version, a panic will occur due to double free.
        // In the corrected version, process() will execute without panic.
        let result = panic::catch_unwind(|| {
            crate::process();
        });
        assert!(
            result.is_ok(),
            "Memory management function panicked due to double free vulnerability"
        );
    }
}
