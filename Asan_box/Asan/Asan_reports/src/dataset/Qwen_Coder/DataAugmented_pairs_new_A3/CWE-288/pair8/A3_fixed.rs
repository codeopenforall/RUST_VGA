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

#[cfg(test)]
mod tests {
    use super::*;

    // Test oracle function: expects the secured debug endpoint to reject invalid tokens.
    #[test]
    fn authentication_oracle() {
        // Instantiate service with debug_mode enabled.
        let svc = Service::new(true);

        // The debug endpoint should enforce authentication and hence reject an invalid token.
        // In the vulnerable implementation, this would incorrectly return true.
        assert!(!svc.process_request("debug", "bad"), "Debug endpoint must not bypass authentication");

        // For a regular API endpoint, an invalid token must fail.
        assert!(!svc.process_request("api", "bad"), "API endpoint with wrong token must fail");

        // Valid token on a non-debug endpoint should succeed.
        assert!(svc.process_request("api", "secret"), "API endpoint with correct token must pass");
    }
}
