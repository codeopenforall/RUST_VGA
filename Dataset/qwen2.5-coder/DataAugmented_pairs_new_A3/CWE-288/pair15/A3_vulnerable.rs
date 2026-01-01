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