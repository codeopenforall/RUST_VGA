use std::collections::HashMap;
use std::str;
struct HttpRequest {
    method: String,
    headers: HashMap<String, String>,
    body: String,
}
impl HttpRequest {
    unsafe fn parse(input: &str) -> Self {
        let header_end = input.find("\r\n\r\n").unwrap_or(input.len());
        let header_str = &input[..header_end];
        let mut headers = HashMap::new();
        let mut method = String::new();
        for (i, line) in header_str.lines().enumerate() {
            if i == 0 {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    method = parts[0].to_string();
                }
            } else {
                if let Some(pos) = line.find(":") {
                    let key = line[..pos].trim();
                    let val = line[pos+1..].trim();
                    headers.insert(key.to_string(), val.to_string());
                }
            }
        }
        let content_length: usize = headers.get("Content-Length")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let body_offset = header_end + 4; 
        let body_str = &input[body_offset..];
        let body = if body_str.len() > content_length {
            body_str[..content_length].to_string()
        } else {
            body_str.to_string()
        };
        HttpRequest { method, headers, body }
    }
}
fn main() {
    let request_str = "GET / HTTP/1.1\r\nContent-Length: 20\r\nHost: example.com\r\n\r\nGET /admin HTTP/1.1\r\n";
    let req = unsafe { HttpRequest::parse(request_str) };
    println!("Method: {}", req.method);
    println!("Body: {}", req.body);
}