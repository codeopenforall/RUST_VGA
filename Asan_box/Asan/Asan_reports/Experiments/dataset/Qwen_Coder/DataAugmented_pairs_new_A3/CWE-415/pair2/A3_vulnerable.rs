pub fn run_simulation() -> usize {
    let raw_ptr = Box::into_raw(Box::new(42));
    // Intentionally causing CWE-415 by dropping the box twice
    unsafe {
        Box::from_raw(raw_ptr);
        Box::from_raw(raw_ptr); // Double free
    }
    2 // Incorrect count to cause test failure
}

#[cfg(test)]
mod tests {
    use super::run_simulation;

    #[test]
    fn test_deallocation() {
        // When run_simulation is executed, it should deallocate the memory exactly once.
        let drop_result = run_simulation();
        assert_eq!(
            drop_result, 
            1, 
            "Expected exactly one drop (proper deallocation). A different count indicates a double free vulnerability."
        );
    }
}
