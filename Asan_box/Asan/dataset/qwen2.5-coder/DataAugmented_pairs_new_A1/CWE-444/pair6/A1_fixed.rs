use std::collections::HashMap;
use std::str;

pub struct HTTPRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub fn parse_input(input: &str) -> Result<HTTPRequest, &'static str> {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    if parts.len() < 2 {
        return Err("Malformed request: missing header/body separator");
    }
    let header_part = parts[0];
    let body_part = parts[1];
    let mut lines = header_part.lines();
    let request_line = lines.next().ok_or("Missing request line")?;
    let req_parts: Vec<&str> = request_line.split_whitespace().collect();
    if req_parts.len() < 3 {
        return Err("Bad request line");
    }
    let mut headers = HashMap::new();
    let mut content_length = None;
    for line in lines {
        let pos = line.find(':').ok_or("Invalid header format")?;
        let key = line[..pos].trim().to_string();
        let value = line[pos + 1..].trim().to_string();
        if key == "Content-Length" {
            if content_length.is_some() {
                return Err("Duplicate Content-Length header");
            }
            content_length = Some(value.parse::<usize>().map_err(|_| "Invalid Content-Length value")?);
        } else {
            headers.insert(key, value);
        }
    }
    if let Some(n) = content_length {
        let body_str = body_part.get(..n).ok_or("Body too short")?;
        return Ok(HTTPRequest {
            method: req_parts[0].to_string(),
            path: req_parts[1].to_string(),
            headers,
            body: body_str.to_string(),
        });
    }
    Ok(HTTPRequest {
        method: req_parts[0].to_string(),
        path: req_parts[1].to_string(),
        headers,
        body: body_part.to_string(),
    })
}

fn main() {
    let request = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 5\r\n\r\nHelloWorld";
    match parse_input(request) {
        Ok(parsed) => {
            println!("Method: {}", parsed.method);
            println!("Path: {}", parsed.path);
            println!("Headers: {:?}", parsed.headers);
            println!("Body: {}", parsed.body);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}