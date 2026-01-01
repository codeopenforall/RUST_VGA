struct Request {
    path: String,
    token: String,
}

struct Server;

impl Server {
    pub fn process_request(&self, req: Request) -> bool {
        static mut GLOBAL_DEBUG: bool = true;
        unsafe {
            if GLOBAL_DEBUG && req.path == "/debug" {
                return true;
            }
        }
        // Simulate a basic authentication check
        if req.token == "valid_token" {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    #[test]
    fn test_authentication() {
        // For a secure system, a request to the debug endpoint with an invalid token must be rejected.
        let server = Server;
        let req = Request {
            path: String::from("/debug"),
            token: String::from("invalid_token"),
        };
        let result = server.process_request(req);
        // In the fixed implementation, authentication is enforced and should return false.
        assert_eq!(result, false, "Unauthorized access: debug endpoint bypassed authentication");
    }
}
