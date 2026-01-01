struct Processor {
    data: String,
}

impl Processor {
    pub fn new(data: String) -> Self {
        Processor { data }
    }

    pub fn process(&self, user_len: usize) -> Result<String, &'static str> {
        let bytes = self.data.as_bytes();
        if user_len > bytes.len() {
            return Err("Input length exceeds data length");
        }
        match thread_shared.process(user_len) {
            Ok(result) => Ok(format!("Processed data: {}", result)),
            Err(e) => Err(e),
        }
    }
}

mod thread_shared {
    pub fn process(user_len: usize) -> Result<String, &'static str> {
        // Simulate some processing
        if user_len > 0 {
            Ok("Success".to_string())
        } else {
            Err("Processing failed")
        }
    }
}