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