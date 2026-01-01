use std::alloc::{Layout, alloc};

struct MemoryManager;

impl MemoryManager {
    const MAX_CAPACITY: usize = 1024;

    unsafe fn reserve(&self, size: usize) -> Result<*mut u8, String> {
        if size > Self::MAX_CAPACITY {
            return Err("Requested allocation size exceeds permitted limit".to_string());
        }
        let layout = Layout::from_size_align(size, 8)
            .map_err(|_| "Invalid memory layout".to_string())?;
        let ptr = alloc(layout);
        if ptr.is_null() {
            return Err("Allocation failed".to_string());
        }
        Ok(ptr)
    }

    pub fn process(&self, size: usize) -> Result<(), String> {
        unsafe {
            let _ptr = self.reserve(size)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Import the definitions from the module containing MemoryManager.
    use super::*;

    // The test verifies that processing a request exceeding the allowed limit fails.
    // In the vulnerable version, process(2000) would succeed (or cause resource issues),
    // while in the corrected version it returns an error.
    #[test]
    fn test_allocation_throttling() {
        let manager = MemoryManager{};
        let result = manager.process(2000);
        assert!(result.is_err(), "Expected error due to allocation size limit, but got success.");
    }
}
