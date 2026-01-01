use std::sync::atomic::{AtomicUsize, Ordering};
static GLOBAL_DROP: AtomicUsize = AtomicUsize::new(0);
struct Resource {
    data: i32,
}
impl Resource {
    fn new(val: i32) -> Self {
        Resource { data: val }
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        GLOBAL_DROP.fetch_add(1, Ordering::SeqCst);
    }
}
fn process_resource() {
    unsafe {
        let boxed = Box::new(Resource::new(100));
        let raw = Box::into_raw(boxed);
        {
            let _holder = Box::from_raw(raw);
        }
    }
}
fn main() {
    process_resource();
    println!("Global drop count: {}", GLOBAL_DROP.load(Ordering::SeqCst));
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    // This test resets the global counter, invokes the process, and asserts the drop count.
    // For the vulnerable version this test will fail (drop count becomes 2),
    // while for the corrected version the drop count will be exactly 1.
    #[test]
    fn test_drop_count() {
        GLOBAL_DROP.store(0, Ordering::SeqCst);
        process_resource();
        let count = GLOBAL_DROP.load(Ordering::SeqCst);
        assert_eq!(count, 1, "Expected drop count of 1, but got {}", count);
    }
}
