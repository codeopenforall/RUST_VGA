fn main() {
    // Simulate some resource management
    let raw = allocate();
    process(raw);
    deallocate(raw);
}

fn allocate() -> *mut u8 {
    // Allocate some memory
    let layout = std::alloc::Layout::new::<u8>();
    unsafe { std::alloc::alloc(layout) }
}

fn process(raw: *mut u8) {
    // Process the allocated memory
    if !raw.is_null() {
        unsafe {
            *raw = 42;
        }
    }
}

fn deallocate(raw: *mut u8) {
    // Deallocate the memory
    if !raw.is_null() {
        let layout = std::alloc::Layout::new::<u8>();
        unsafe { std::alloc::dealloc(raw, layout) };
    }
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
