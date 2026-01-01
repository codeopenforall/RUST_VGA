use std::collections::HashMap;

struct Service {
    approved: Vec<String>,
}

impl Service {
    fn new(approved: Vec<String>) -> Self {
        Service { approved }
    }

    pub fn process(&self, req: Request) -> Result<String, &'static str> {
        let origin = req.headers.get("Origin").ok_or("Origin header missing")?;
        if !self.approved.contains(origin) {
            return Err("Origin not authorized");
        }
        let mut url = "http://".to_owned();
        url.push_str(origin);
        Ok(url)
    }
}

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    fn new(headers: HashMap<String, String>, body: String) -> Self {
        Request { headers, body }
    }
}