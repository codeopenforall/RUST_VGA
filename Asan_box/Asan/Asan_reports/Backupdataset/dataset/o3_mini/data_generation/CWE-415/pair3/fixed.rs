use std::sync::atomic::{AtomicBool, Ordering};
use std::ptr;
use std::mem;
use std::boxed::Box;
static FREED: AtomicBool = AtomicBool::new(false);
pub struct Data {
    pub value: i32,
}
impl Data {
    pub fn new(val: i32) -> Self {
        Data { value: val }
    }
}
unsafe fn deallocate(ptr: *mut Data) {
    if FREED.swap(true, Ordering::SeqCst) {
        panic!("Double free detected!");
    }
    Box::from_raw(ptr);
}
fn obtain_pointer() -> *mut Data {
    let data = Box::new(Data::new(42));
    Box::into_raw(data)
}
fn process() {
    let raw = obtain_pointer();
    unsafe {
        deallocate(raw);
    }
}
fn main() {
    process();
}


#[cfg(test)]
mod tests {
    use std::panic;

    #[test]
    fn resource_management_test() {
        // When executing the code, the fixed version should run without panicking.
        // In the vulnerable version, the double free will trigger a panic.
        let result = panic::catch_unwind(|| {
            // Invokes the main function which in turn calls process().
            crate::main();
        });
        assert!(
            result.is_ok(),
            "Test failed: the process triggered a double free panic!"
        );
    }
}
