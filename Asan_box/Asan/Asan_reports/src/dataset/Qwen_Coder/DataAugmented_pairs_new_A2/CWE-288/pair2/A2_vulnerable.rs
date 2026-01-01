use std::sync::{Arc, Mutex};

static mut GLOBAL_DEBUG: bool = true;

struct Request {
    endpoint: String,
    token: Option<String>,
}

struct Server {
    secret: String,
    config: Arc<Mutex<Config>>,
}

struct Config {
    require_auth: bool,
}

trait Processor {
    fn process_request(&self, req: &Request) -> bool;
}

impl Server {
    fn new(secret: &str, require_auth: bool) -> Self {
        Server {
            secret: secret.to_string(),
            config: Arc::new(Mutex::new(Config { require_auth })),
        }
    }
}

impl Processor for Server {
    fn process_request(&self, req: &Request) -> bool {
        if req.endpoint == "/admin" {
            let cfg = self.config.lock().unwrap();
            if cfg.require_auth {
                return req.token.as_ref() == Some(&self.secret);
            }
            return true;
        } else if req.endpoint == "/debug" {
            unsafe {
                if GLOBAL_DEBUG {
                    return true;
                }
            }
            // Defect: Incorrectly returns true instead of checking the token
            return true;
        }
        false
    }
}

fn main() {
    let server = Server::new("supersecret", true);
    let req_admin = Request {
        endpoint: "/admin".to_string(),
        token: Some("supersecret".to_string()),
    };
    let req_debug = Request {
        endpoint: "/debug".to_string(),
        token: None,
    };
    println!("Admin endpoint access: {}", server.process_request(&req_admin));
    println!("Debug endpoint access: {}", server.process_request(&req_debug));
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
