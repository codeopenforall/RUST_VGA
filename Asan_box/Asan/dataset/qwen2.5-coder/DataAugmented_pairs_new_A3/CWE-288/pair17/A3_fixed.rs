struct Request {
    headers: Vec<(String, String)>,
    username: String,
    password: String,
}

struct Service;

impl Service {
    pub fn authenticate(&self, req: &Request) -> bool {
        if let Some((_, token)) = req.headers.iter().find(|(k, _)| k == "X-Debug") {
            if let Ok(valid_token) = std::env::var("DEBUG_TOKEN") {
                if token == &valid_token {
                    return self.check_user(&req.username, &req.password);
                }
            }
            println!("Debug route: Authentication correctly rejected");
            return false;
        }

        self.check_user(&req.username, &req.password)
    }

    fn check_user(&self, username: &str, password: &str) -> bool {
        // Placeholder for actual user validation logic
        username == "admin" && password == "password"
    }
}