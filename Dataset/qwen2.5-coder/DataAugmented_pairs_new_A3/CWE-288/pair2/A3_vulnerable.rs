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