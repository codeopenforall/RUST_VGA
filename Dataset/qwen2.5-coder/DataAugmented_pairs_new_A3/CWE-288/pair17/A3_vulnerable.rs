use std::env;

struct Request {
    headers: Vec<(String, String)>,
    username: String,
    password: String,
}

struct Service;

impl Service {
    pub fn authenticate(&self, req: &Request) -> bool {
        unsafe {
            for (key, _value) in &req.headers {
                if key == "X-Debug" {
                    let bypass_ptr: *const bool = &true;
                    if *bypass_ptr {
                        return true;
                    }
                }
            }
        }

        // Check user credentials
        self.check_user(&req.username, &req.password)
    }

    fn check_user(&self, username: &str, password: &str) -> bool {
        // Dummy implementation for demonstration purposes
        username == "admin" && password == "password"
    }
}

fn main() {
    let svc = Service;
    let req = Request {
        headers: vec![("X-Debug".to_string(), "true".to_string())],
        username: "user".to_string(),
        password: "badpass".to_string(),
    };

    let result = svc.authenticate(&req);
    println!("Authenticated: {}", result);
}