use std::thread;

struct BufferHandler {
    buffer: Vec<u32>,
}

impl BufferHandler {
    fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        unsafe { buffer.set_len(size); } // Corrected size allocation
        for i in 0..size {
            buffer[i] = 0;
        }
        Self { buffer }
    }

    fn update(&mut self, index: usize, value: u32) {
        if index < self.buffer.len() { // Added bounds check
            unsafe {
                let ptr = self.buffer.as_mut_ptr();
                *ptr.add(index) = value;
            }
        }
    }

    fn aggregate(&self) -> u32 {
        self.buffer.iter().sum()
    }
}

fn process() -> u32 {
    let mut handler = BufferHandler::new(10);
    let join_handle = thread::spawn(move || {
        handler.update(10, 42); // This will now be ignored due to bounds check
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
