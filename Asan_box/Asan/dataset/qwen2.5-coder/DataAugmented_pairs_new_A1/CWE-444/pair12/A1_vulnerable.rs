use std::collections::HashMap;
use std::str;

struct HttpRequest {
    headers: HashMap<String, String>,
    content: String,
}

impl HttpRequest {
    unsafe fn extract_headers(input: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for line in input.split("\r\n") {
            if let Some(colon_index) = line.find(':') {
                let key = line[..colon_index].trim();
                let value = line[colon_index + 1..].trim();
                let k = String::from_utf8_unchecked(key.as_bytes().to_vec());
                let v = String::from_utf8_unchecked(value.as_bytes().to_vec());
                headers.insert(k, v);
            }
        }
        headers
    }

    fn parse(raw: &str) -> Option<Self> {
        let parts: Vec<&str> = raw.split("\r\n\r\n").collect();
        if parts.len() < 2 {
            return None;
        }
        let header_section = parts[0];
        let body_section = parts[1];
        let headers = unsafe { Self::extract_headers(header_section) };
        let content = if let Some(content_length) = headers.get("Content-Length") {
            if let Ok(length) = content_length.parse::<usize>() {
                if body_section.len() >= length {
                    body_section[..length].to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else if headers.get("Transfer-Encoding")
                        .map(|v| v.to_lowercase() == "chunked")
                        .unwrap_or(false) {
            HttpRequest::decode_chunks(body_section)
        } else {
            body_section.to_string()
        };
        Some(HttpRequest { headers, content })
    }

    fn decode_chunks(input: &str) -> String {
        let mut result = String::new();
        let mut lines = input.lines();
        loop {
            let size_line = match lines.next() {
                Some(line) => line,
                None => break,
            };
            let chunk_size = match usize::from_str_radix(size_line, 16) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            let mut chunk = String::new();
            let mut read_bytes = 0;
            while read_bytes < chunk_size {
                if let Some(line) = lines.next() {
                    let len = line.len();
                    chunk.push_str(line);
                    read_bytes += len;
                } else {
                    break;
                }
            }
            result.push_str(&chunk);
        }
        result
    }
}

fn main() {
    let input = "POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 5\r\nTransfer-Encoding: chunked\r\n\r\n7\r\nPayloadData\r\n0\r\n\r\n";
    if let Some(req) = HttpRequest::parse(input) {
        println!("Processed content length: {}", req.content.len());
    } else {
        println!("Failed to parse request");
    }
}