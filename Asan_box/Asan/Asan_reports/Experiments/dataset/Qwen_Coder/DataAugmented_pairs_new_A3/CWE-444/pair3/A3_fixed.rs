use std::str;

#[derive(Debug)]
struct Request {
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Request {
    pub fn new(raw: &[u8]) -> Option<Self> {
        let raw_str = std::str::from_utf8(raw).ok()?;
        let mut headers = Vec::new();
        let mut lines = raw_str.split("\r\n");
        let _ = lines.next()?;
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                continue;
            }
            headers.push((parts[0].trim().to_string(), parts[1].trim().to_string()));
        }
        let body_marker = "\r\n\r\n";
        let body_start = raw_str.find(body_marker).map(|p| p + body_marker.len()).unwrap_or(raw.len());
        let body = raw[body_start..].to_vec();
        Some(Request { headers, body })
    }
}

fn process(req: &Request) -> Result<&str, &str> {
    let cnt_length = req.headers.iter()
        .find(|&&(ref k, _)| k.eq_ignore_ascii_case("Content-Length"))
        .and_then(|(_, v)| v.parse::<usize>().ok());

    if let Some(expected) = cnt_length {
        if expected != req.body.len() {
            return Err("Body length mismatch");
        }
    }

    Ok("Request processed")
}

//////////////////////////
// Test Oracle Function
//////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_request_smuggling() {
        // Crafted HTTP request with consistent Content-Length header
        // The intended body "Hello World" is 11 bytes long.
        let request_data = b"GET / HTTP/1.1\r\nContent-Length: 11\r\n\r\nHello World";
        
        // Parse the request using the provided functions.
        let req = Request::new(request_data).expect("Parsing failed");
        let outcome = process(&req);
        
        // For the vulnerable version, due to an off-by-one error,
        // the body length is misinterpreted and request validation fails.
        // For the corrected version, it should pass.
        assert_eq!(outcome, Ok("Request processed"), "Request processing failed");
    }
}
