mod server {
    use std::sync::Arc;
    use std::thread;
    #[derive(Debug)]
    pub struct Request {
        pub header: String,
        pub body: Vec<u8>,
    }
    impl Request {
        pub fn parse(raw: &[u8]) -> Result<Request, &'static str> {
            let text = std::str::from_utf8(raw).map_err(|_| "Invalid UTF8 in request")?;
            let parts: Vec<&str> = text.split("\r\n\r\n").collect();
            if parts.len() < 2 {
                return Err("Malformed HTTP request");
            }
            let header = parts[0].to_string();
            let body = parts[1].as_bytes().to_vec();
            Ok(Request { header, body })
        }
    }
    pub fn process_request(raw: &[u8]) -> Result<usize, &'static str> {
        let req = Request::parse(raw)?;
        let mut cl_values = Vec::new();
        for line in req.header.lines() {
            if line.to_lowercase().starts_with("content-length:") {
                let value_part = line.split(':').nth(1).ok_or("Missing header value")?;
                let trimmed = value_part.trim();
                let val = trimmed.parse::<usize>().map_err(|_| "Invalid number")?;
                cl_values.push(val);
            }
        }
        if cl_values.is_empty() {
            return Err("Missing Content-Length header");
        }
        if !cl_values.windows(2).all(|w| w[0] == w[1]) {
            return Err("Inconsistent Content-Length headers");
        }
        if cl_values[0] != req.body.len() {
            return Err("Body length does not match Content-Length");
        }
        Ok(cl_values[0])
    }
    pub fn start(data: Vec<u8>) {
        let arc_data = Arc::new(data);
        let thread_data = arc_data.clone();
        let handle = thread::spawn(move || {
            let res = process_request(&thread_data);
            match res {
                Ok(len) => println!("Processed body. Confirmed Content-Length: {}", len),
                Err(e) => println!("Error: {}", e),
            }
        });
        handle.join().unwrap();
    }
}
fn main() {
    let req = b"POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 11\r\n\r\nhello world";
    server::start(req.to_vec());
}

#[cfg(test)]
mod tests {
    use super::server;
    
    // Test oracle: Send a malicious HTTP request with duplicate, conflicting Content-Length headers.
    // For the vulnerable version, the processing function accepts the first header (5) leading to an
    // inconsistent interpretation (since the actual body length is 11). The test expects an error.
    // For the corrected version, the function properly rejects the request due to inconsistent headers.
    #[test]
    fn test_request_smuggling() {
        // Duplicate headers: first is 5, second is 10 but actual body "hello world" is 11 bytes.
        let malicious_req = b"POST / HTTP/1.1\r\nHost: test\r\nContent-Length: 5\r\nContent-Length: 10\r\n\r\nhello world";
        
        // The contract: processing must return an error.
        let res = unsafe { server::process_request(malicious_req) };
        assert!(res.is_err(), "Request with duplicate conflicting headers must be rejected");
    }
}
