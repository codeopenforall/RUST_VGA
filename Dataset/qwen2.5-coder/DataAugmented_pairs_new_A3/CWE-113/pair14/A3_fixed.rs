use std::sync::{Arc, Mutex};

struct HeaderGenerator {
    header: Arc<Mutex<String>>,
}

impl HeaderGenerator {
    fn new() -> Self {
        HeaderGenerator {
            header: Arc::new(Mutex::new(String::new())),
        }
    }

    fn sanitize(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }

    fn build_header(&self, input: &str) {
        let safe_input = HeaderGenerator::sanitize(input);
        let mut response = String::with_capacity(safe_input.len());
        response.push_str(&safe_input);

        let mut guard = self.header.lock().unwrap();
        guard.push_str(&response);
    }

    fn output(&self) -> String {
        self.header.lock().unwrap().clone()
    }
}