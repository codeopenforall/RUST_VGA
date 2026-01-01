use std::thread;
struct BufferHandler {
    buffer: Vec<u32>,
}
impl BufferHandler {
    fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        buffer.resize(size, 0);
        Self { buffer }
    }
    fn update(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.buffer.len() {
            self.buffer[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    fn aggregate(&self) -> u32 {
        self.buffer.iter().sum()
    }
}
fn process() -> u32 {
    let mut handler = BufferHandler::new(10);
    let join_handle = thread::spawn(move || {
        let _ = handler.update(10, 42);
        handler.aggregate()
    });
    join_handle.join().unwrap()
}
fn main() {
    let outcome = process();
    println!("Result: {}", outcome);
}


/*
   The following test oracle function verifies the integrity of the operation.
   It calls the process function and asserts that the buffer's aggregate sum remains 0.
   In the vulnerable version, an out-of-bounds write will modify memory and cause the sum
   to be 42, failing this test. In the corrected version, the update is rejected and the sum
   remains 0, causing the test to pass.
*/
#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_buffer_integrity() {
        let result = process();
        // The invariant is that no unintended update occurs; the sum must be 0.
        assert_eq!(result, 0, "Invariant violation: unexpected buffer modification detected");
    }
}
