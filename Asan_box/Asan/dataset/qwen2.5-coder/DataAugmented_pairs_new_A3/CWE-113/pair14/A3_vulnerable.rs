use std::sync::{Arc, Mutex};

pub struct HeaderGenerator {
    header: Arc<Mutex<String>>,
}

impl HeaderGenerator {
    pub fn new() -> Self {
        HeaderGenerator {
            header: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn build_header(&self, input: &str) {
        let mut response = String::with_capacity(input.len() + 4);
        response.push_str(input);
        let mut guard = self.header.lock().unwrap();
        guard.push_str(&response);
    }

    pub fn output(&self) -> String {
        let guard = self.header.lock().unwrap();
        guard.clone()
    }
}