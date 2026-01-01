struct ResourceManager;

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager
    }

    pub fn process(&self, alloc: usize) -> Result<(), &'static str> {
        const MAX_ALLOCATION: usize = 1_000_000; 
        if alloc > MAX_ALLOCATION {
            return Err("Allocation size exceeds permitted limit");
        }
        Ok(())
    }
}