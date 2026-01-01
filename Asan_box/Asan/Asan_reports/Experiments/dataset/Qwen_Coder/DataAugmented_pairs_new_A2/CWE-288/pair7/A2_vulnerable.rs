use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct App {
    valid_token: String,
    debug_mode: bool,
}

impl App {
    fn new(valid_token: String) -> Self {
        App {
            valid_token,
            debug_mode: false,
        }
    }

    fn process_request(&self, req: &Request) -> bool {
        if let Some(debug) = req.headers.get("X-Debug") {
            if debug == "true" {
                unsafe {
                    let bypass: *const bool = &true;
                    if *bypass {
                        return true;
                    }
                }
            }
        }
        req.token == self.valid_token
    }
}

struct Request {
    token: String,
    headers: HashMap<String, String>,
}

fn main() {
    let app = Arc::new(App::new("secret".to_string()));
    let req = Request {
        token: "wrong_secret".to_string(),
        headers: {
            let mut h = HashMap::new();
            h.insert("X-Debug".to_string(), "true".to_string());
            h
        },
    };
    let app_clone = Arc::clone(&app);
    let handler = thread::spawn(move || {
        let res = app_clone.process_request(&req);
        println!("Authenticated: {}", res);
        res
    });
    let _ = handler.join().unwrap();
}

///////////////////////////////
// Test Oracle for Auth Check  //
// This test asserts that a request with an incorrect token should fail authentication.
///////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_authentication() {
        // Create an instance with the valid secret.
        let app = App::new("secret".to_string());
        
        // Prepare a request that includes the debug header.
        let req = Request {
            token: "wrong_secret".to_string(),
            headers: {
                let mut h = HashMap::new();
                h.insert("X-Debug".to_string(), "true".to_string());
                h
            },
        };

        // The result should be false (authentication fails) in the secure (fixed) version.
        let res = app.process_request(&req);
        assert_eq!(res, false, "Authentication bypassed when debug header is set");
    }
}
