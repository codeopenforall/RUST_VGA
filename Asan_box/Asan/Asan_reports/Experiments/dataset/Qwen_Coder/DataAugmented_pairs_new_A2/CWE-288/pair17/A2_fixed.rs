use std::env;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: Vec<(String, String)>,
    username: String,
    password: String,
}

struct Service;

impl Service {
    fn authenticate(&self, req: &Request) -> bool {
        if let Some((_, token)) = req.headers.iter().find(|(k, _)| k == "X-Debug") {
            if let Ok(valid_token) = env::var("DEBUG_TOKEN") {
                if token == &valid_token {
                    return self.check_user(&req.username, &req.password);
                }
            }
            return false;
        }
        self.check_user(&req.username, &req.password)
    }

    fn check_user(&self, username: &str, password: &str) -> bool {
        username == "admin" && password == "password"
    }
}

fn main() {
    let service = Arc::new(Service);
    let service_thread = Arc::clone(&service);
    let handle = thread::spawn(move || {
        let req = Request {
            headers: vec![("X-Debug".to_string(), "wrong".to_string())],
            username: "user".to_string(),
            password: "badpass".to_string(),
        };
        if service_thread.authenticate(&req) {
            println!("Debug route: Authentication succeeded (unexpected)");
        } else {
            println!("Debug route: Authentication correctly rejected");
        }
    });
    handle.join().unwrap();
    let req2 = Request {
        headers: vec![],
        username: "admin".to_string(),
        password: "password".to_string(),
    };
    if service.authenticate(&req2) {
        println!("Normal route: Authentication succeeded");
    } else {
        println!("Normal route: Authentication failed");
    }
}

//////////////////////////////////////////////
// Test Oracle Function
//
// The following test function is used to verify that the vulnerable code bypasses authentication
// when a debug header is present, while the corrected version enforces proper authentication.
//
// For both versions, a Request with the "X-Debug" header and an incorrect token should fail authentication.
// Also, a regular request with valid credentials should succeed.
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authentication() {
        let svc = Service;
        // Test case: Debug header is present with an incorrect token.
        // In the vulnerable code, this would bypass auth and return true.
        // In the fixed code, this should return false.
        let req_debug = Request {
            headers: vec![("X-Debug".to_string(), "wrong".to_string())],
            username: "user".to_string(),
            password: "badpass".to_string(),
        };

        // Expectation: Authentication should fail for debug requests without a valid token.
        let result = svc.authenticate(&req_debug);
        assert!(!result, "Authentication bypass occurred via debug header!");

        // Normal authentication: Only valid credentials should pass.
        let req_normal = Request {
            headers: vec![],
            username: "admin".to_string(),
            password: "password".to_string(),
        };
        let result_normal = svc.authenticate(&req_normal);
        assert!(result_normal, "Valid authentication failed!");
    }
}
