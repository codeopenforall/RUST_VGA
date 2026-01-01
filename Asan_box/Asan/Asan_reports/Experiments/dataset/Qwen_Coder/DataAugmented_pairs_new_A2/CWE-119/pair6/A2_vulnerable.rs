use std::thread;
struct MemoryHandler {
    buffer: *mut u32,
    capacity: usize,
}
impl MemoryHandler {
    pub fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 0);
        let boxed = vec.into_boxed_slice();
        let ptr = Box::into_raw(boxed) as *mut u32;
        MemoryHandler {
            buffer: ptr,
            capacity: size,
        }
    }
    pub unsafe fn write_unchecked(&self, index: usize, value: u32) {
        *self.buffer.add(index) = value;
    }
    pub unsafe fn read(&self, index: usize) -> u32 {
        *self.buffer.add(index)
    }
    pub fn free(self) {
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(self.buffer, self.capacity));
        }
    }
}
pub fn process_operation(index: usize) -> Result<u32, &'static str> {
    let handler = MemoryHandler::new(10);
    unsafe {
        handler.write_unchecked(index, 99);
        let res = handler.read(index);
        handler.free();
        Ok(res)
    }
}
fn main() {
    let handle = thread::spawn(|| {
        let r = process_operation(5);
        println!("Thread operation result: {:?}", r);
    });
    let result = process_operation(10);
    handle.join().unwrap();
    println!("Main thread operation result: {:?}", result);
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
