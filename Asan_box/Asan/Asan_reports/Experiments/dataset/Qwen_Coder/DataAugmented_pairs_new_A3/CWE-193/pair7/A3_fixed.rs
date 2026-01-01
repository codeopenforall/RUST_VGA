struct BufferHandler {
    buffer: Vec<i32>,
}

impl BufferHandler {
    pub fn new() -> Self {
        BufferHandler { buffer: Vec::new() }
    }

    pub fn load(&mut self, data: &[i32]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn calculate(&self) -> i32 {
        let len = self.buffer.len();
        let mut sum = 0;
        for i in 0..len {
            sum += self.buffer[i];
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // This oracle test is designed so that:
    // - In the vulnerable version, the off-by-one error will cause out-of-bound access
    //   and the test will panic or produce an incorrect sum.
    // - In the fixed version, the sum will be computed correctly as 10.
    #[test]
    fn test_buffer_sum() {
        let mut handler = BufferHandler::new();
        // Input array: expected sum is 1 + 2 + 3 + 4 = 10.
        handler.load(&[1, 2, 3, 4]);
        let shared_handler = Arc::new(handler);
        let handler_clone = Arc::clone(&shared_handler);
        let worker = thread::spawn(move || handler_clone.calculate());
        let result = worker.join().expect("Worker thread panicked");
        assert_eq!(result, 10, "The computed sum did not match the expected value.");
    }
}
