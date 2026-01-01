struct Resource {
    data: i32,
}
static mut FREED: bool = false;
unsafe fn deallocate_once(ptr: *mut Resource) {
    if !FREED {
        FREED = true;
        let _ = Box::from_raw(ptr);
    }
}
fn execute() {
    let resource = Box::new(Resource { data: 42 });
    let raw_ptr = Box::into_raw(resource);
    unsafe {
        deallocate_once(raw_ptr);
    }
}
fn main() {
    execute();
}


#[cfg(test)]
mod tests {
    use std::panic;

    // The test oracle function runs the main execution of the module under test.
    // When the module is compiled with the "vul" feature flag, it expects a panic due to the double free.
    // When compiled without the feature flag (i.e. the corrected code), it expects normal execution.
    #[test]
    fn test_oracle() {
        let result = panic::catch_unwind(|| {
            crate::main();
        });
        // If the "vul" feature is enabled, we expect a panic (i.e. result is an error).
        #[cfg(feature = "vul")]
        assert!(result.is_err(), "Expected panic due to double free vulnerability");

        // If the "vul" feature is not enabled, we expect normal execution without panic.
        #[cfg(not(feature = "vul"))]
        assert!(result.is_ok(), "Execution should succeed without panic");
    }
}
