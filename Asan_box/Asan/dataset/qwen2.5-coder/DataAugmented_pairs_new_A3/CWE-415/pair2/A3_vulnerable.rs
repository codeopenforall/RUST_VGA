pub fn run_simulation() -> usize {
    let raw_ptr = Box::into_raw(Box::new(42));
    // Intentionally causing CWE-415 by dropping the box twice
    unsafe {
        Box::from_raw(raw_ptr);
        Box::from_raw(raw_ptr); // Double free
    }
    2 // Incorrect count to cause test failure
}