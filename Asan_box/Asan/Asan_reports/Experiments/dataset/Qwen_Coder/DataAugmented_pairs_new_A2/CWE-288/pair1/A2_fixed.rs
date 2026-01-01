use std::sync::Arc;
use std::thread;

struct Request {
    path: String,
    token: Option<String>,
}

impl Request {
    fn new(path: &str, token: Option<&str>) -> Self {
        Request {
            path: path.to_string(),
            token: token.map(|s| s.to_string()),
        }
    }
}

struct Application {
    debug_bypass: bool,
}

impl Application {
    fn new(debug_bypass: bool) -> Self {
        Application { debug_bypass }
    }

    fn verify(&self, req: &Request) -> bool {
        if req.path == "/debug" && !self.debug_bypass {
            return false;
        }
        if let Some(ref token) = req.token {
            return token == "secret";
        }
        false
    }
}

fn main() {
    let app = Arc::new(Application::new(false));
    let req = Request::new("/debug", None);
    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        if app_clone.verify(&req) {
            println!("Access granted!");
        } else {
            println!("Access denied!");
        }
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication() {
        // Test for requests to the debug endpoint without a token.
        // In secure configuration, authentication should fail.
        let req = Request::new("/debug", None);
        
        // Simulate production configuration.
        let app_secure = Application::new(false);
        let secure_result = app_secure.verify(&req);
        assert!(!secure_result, "Access should be denied for debug endpoint without a token in secure configuration");
        
        // For comparison, the vulnerable configuration (debug_bypass enabled) returns true.
        let app_insecure = Application::new(true);
        let insecure_result = app_insecure.verify(&req);
        assert!(!insecure_result, "Test oracle expects failure; vulnerable implementation incorrectly grants access");
    }
}
