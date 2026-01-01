struct MemoryHandler {
    buffer: *mut u32,
    capacity: usize,
}

impl MemoryHandler {
    pub fn new(capacity: usize) -> Self {
        let buffer = unsafe { std::alloc::alloc(std::alloc::Layout::array::<u32>(capacity).unwrap()) as *mut u32 };
        MemoryHandler { buffer, capacity }
    }

    pub fn write_checked(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.capacity {
            unsafe {
                *self.buffer.add(index) = value;
            }
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub unsafe fn read(&self, index: usize) -> u32 {
        *self.buffer.add(index)
    }

    pub fn free(self) {
        unsafe {
            std::alloc::dealloc(self.buffer as *mut u8, std::alloc::Layout::array::<u32>(self.capacity).unwrap());
        }
    }
}

pub fn process_operation(index: usize) -> Result<u32, &'static str> {
    let mut handler = MemoryHandler::new(10);
    handler.write_checked(index, 99)?;
    let res = unsafe { handler.read(index) };
    handler.free();
    Ok(res)
}

//////////////////////////
// Test Oracle Function
//////////////////////////
#[cfg(test)]
mod tests {
    // Import the module under test. For the vulnerable build, this module will exhibit unsafe behavior.
    // For the fixed build, an out-of-bound operation returns an error.
    use super::process_operation;

    #[test]
    fn test_memory_bounds() {
        // Trigger the operation with an out-of-bound index.
        let op_result = process_operation(10);
        // The correct (fixed) behavior is to return an error when accessing an invalid index.
        // If the implementation is vulnerable, it will return Ok(99) (thus failing the test).
        assert!(
            op_result.is_err(),
            "Test failed: Operation should not succeed with an out-of-bound index."
        );
    }
}
