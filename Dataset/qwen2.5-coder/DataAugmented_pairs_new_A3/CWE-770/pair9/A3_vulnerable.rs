struct Allocator;

impl Allocator {
    pub fn new() -> Self {
        Allocator
    }

    pub fn allocate_with_limit(&self, n: usize) -> Result<(), &'static str> {
        // Intentionally remove the check for maximum allocation size
        // This will cause the test to fail as it allows unbounded allocation
        Ok(())
    }
}