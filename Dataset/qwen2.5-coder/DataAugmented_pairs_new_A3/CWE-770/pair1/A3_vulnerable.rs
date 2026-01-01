struct ResourceManager;

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager
    }

    pub fn process(&self, alloc: usize) -> Result<(), &'static str> {
        // Intentionally remove the allocation size check to cause a test failure
        // if alloc > MAX_ALLOCATION {
        //     return Err("Allocation size exceeds permitted limit");
        // }
        Ok(())
    }
}