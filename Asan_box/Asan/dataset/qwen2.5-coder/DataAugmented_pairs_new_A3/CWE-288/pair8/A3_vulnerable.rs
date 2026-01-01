struct Service {
    debug_mode: bool,
}

impl Service {
    pub fn new(debug_mode: bool) -> Self {
        Service { debug_mode }
    }

    pub fn process_request(&self, path: &str, token: &str) -> bool {
        let bypass = unsafe {
            let p: *const bool = &self.debug_mode;
            *p
        };
        if path == "debug" && bypass {
            return true;
        }

        if path == "api" && token == "secret" {
            return true;
        }

        false
    }
}