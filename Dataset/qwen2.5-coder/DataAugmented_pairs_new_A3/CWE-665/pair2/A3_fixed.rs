struct Request {
    endpoint: String,
    token: Option<String>,
}

struct Server {
    secret: String,
    require_auth: bool,
}

impl Server {
    fn new(secret: &str, require_auth: bool) -> Self {
        Server {
            secret: secret.to_string(),
            require_auth,
        }
    }

    fn process_request(&self, req: &Request) -> bool {
        if req.endpoint == "/admin" || req.endpoint == "/debug" {
            if self.require_auth {
                return req.token.as_ref() == Some(&self.secret);
            }
        }
        true
    }
}