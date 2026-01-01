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