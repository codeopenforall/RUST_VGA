struct BufferManager {
    data: Vec<u8>,
}

impl BufferManager {
    fn get_segment_safe(&self, start: usize, length: usize) -> Result<&str, &'static str> {
        let end = start.checked_add(length).ok_or("overflow in parameters")?;
        if end > self.data.len() {
            return Err("out of bounds");
        }
        let slice = &self.data[start..end];
        std::str::from_utf8(slice).map_err(|_| "invalid utf8")
    }
}

struct Processor {
    manager: std::sync::Arc<std::sync::Mutex<BufferManager>>,
}

impl Processor {
    fn run(&self, start: usize, length: usize) -> Result<&str, &'static str> {
        let manager = self.manager.lock().unwrap();
        manager.get_segment_safe(start, length)
    }
}

//////////////////////
// Test Oracle Code
//////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // This test verifies that when provided with out-of-bound parameters for the input string,
    // the safe implementation correctly returns an error.
    // For the vulnerable version, this test is expected to fail (panic or undefined behavior).
    #[test]
    fn test_oracle() {
        // "12345" has length 5; parameters (start=3, length=5) result in end=8, which is out-of-bound.
        let input_data = "12345".to_string();
        let manager = BufferManager { data: input_data.into_bytes() };
        let proc_inst = Processor { manager: Arc::new(Mutex::new(manager)) };
        let result = proc_inst.run(3, 5);
        assert!(result.is_err(), "Expected an error due to out-of-bound extraction");
        assert_eq!(result.err().unwrap(), "out of bounds");
    }
}
