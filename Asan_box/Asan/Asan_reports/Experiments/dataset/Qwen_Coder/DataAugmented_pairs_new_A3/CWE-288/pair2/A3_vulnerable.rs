struct Server {
    secret: String,
    debug_mode: bool,
}

struct Request {
    endpoint: String,
    token: Option<String>,
}

impl Server {
    pub fn new(secret: &str, debug_mode: bool) -> Self {
        Server {
            secret: secret.to_string(),
            debug_mode,
        }
    }

    pub fn process_request(&self, req: &Request) -> bool {
        static mut GLOBAL_DEBUG: bool = true;

        if req.endpoint == "/admin" {
            return false;
        } else if req.endpoint == "/debug" {
            unsafe {
                if GLOBAL_DEBUG {
                    return true;
                }
            }
            return req.token.as_ref() == Some(&self.secret);
        }

        false
    }
}

//////////////////////////////////////////////
// Test Oracle Function for Authentication Check
// This test should fail when run against the vulnerable version (debug endpoint bypasses auth)
// and pass with the fixed version (debug endpoint requires authentication).
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authentication_oracle() {
        // Initialize the server in authentication-required mode.
        let server = Server::new("supersecret", true);
        // Prepare a request to the debug endpoint without a token.
        let debug_request = Request {
            endpoint: "/debug".to_string(),
            token: None,
        };
        // The corrected behavior (fixed version) should reject the request.
        // Vulnerable version would incorrectly grant access.
        let access_granted = server.process_request(&debug_request);
        assert_eq!(
            access_granted,
            false,
            "Unauthorized access to the debug endpoint should be denied"
        );
    }
}
