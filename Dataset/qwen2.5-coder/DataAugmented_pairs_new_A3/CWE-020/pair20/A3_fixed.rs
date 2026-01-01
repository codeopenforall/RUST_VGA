struct Processor {
    buffer: Vec<u8>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            buffer: vec![0; 10], // Example buffer size
        }
    }

    pub fn process(&self, len_str: &str) -> String {
        match len_str.parse::<usize>() {
            Ok(len) => {
                // FIX: Validate that the provided length does not exceed the buffer size.
                if len > self.buffer.len() {
                    return "Invalid input size".to_string();
                }
                // Safe conversion: Using pointer arithmetic only after ensuring bounds.
                let ptr = self.buffer.as_ptr();
                let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
                // Further processing can be done here with the slice
                format!("Processed {} bytes", slice.len())
            }
            Err(_) => "Invalid input format".to_string(),
        }
    }
}