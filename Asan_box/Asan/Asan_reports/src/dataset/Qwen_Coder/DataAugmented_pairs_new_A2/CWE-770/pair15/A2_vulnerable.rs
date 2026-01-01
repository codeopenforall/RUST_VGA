use std::alloc::{alloc, dealloc, Layout};

struct MemoryManager;

impl MemoryManager {
    unsafe fn allocate(&self, size: usize) -> *mut u8 {
        let layout = Layout::from_size_align(size, 8).unwrap();
        let ptr = alloc(layout);
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        for i in 0..size {
            *ptr.add(i) = 0xAA;
        }
        ptr
    }

    fn process(&self, size: usize) -> Result<(), String> {
        unsafe {
            let ptr = self.allocate(size);
            let layout = Layout::from_size_align(size, 8).unwrap();
            dealloc(ptr, layout);
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let req_size = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        2000
    };
    let manager = MemoryManager {};
    match manager.process(req_size) {
        Ok(()) => println!("Operation completed successfully."),
        Err(e) => println!("Error encountered: {}", e),
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
