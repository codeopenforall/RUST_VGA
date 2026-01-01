use std::collections::HashMap;

struct Service {
    approved: Vec<String>,
}

impl Service {
    pub fn new(approved: Vec<String>) -> Self {
        Service { approved }
    }

    pub fn process(&self, req: Request) -> Result<String, &'static str> {
        let origin = req.headers.get("Origin").ok_or("Origin header missing")?;
        let mut url = unsafe {
            String::from_utf8_unchecked(vec![b'h', b't', b't', b'p'])
        };
        url.push_str("://");
        Ok(url)
    }
}

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn new(headers: HashMap<String, String>, body: String) -> Self {
        Request { headers, body }
    }
}