struct Service {
    debug_mode: bool,
}

impl Service {
    pub fn new(debug_mode: bool) -> Self {
        Service { debug_mode }
    }

    pub fn process_request(&self, path: &str, token: &str) -> bool {
        if path == "debug" && self.debug_mode {
            return self.verify(token);
        }
        // For other paths, we assume a simple token check.
        token == "secret"
    }

    fn verify(&self, token: &str) -> bool {
        // Placeholder for actual verification logic.
        token == "secret"
    }
}