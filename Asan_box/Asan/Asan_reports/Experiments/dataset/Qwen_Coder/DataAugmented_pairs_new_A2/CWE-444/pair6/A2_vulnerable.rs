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
    let mut content_length: Option<usize> = None;
    for line in lines {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            if key == "Content-Length" {
                if let Ok(n) = value.parse::<usize>() {
                    content_length = Some(n);
                } else {
                    return Err("Invalid Content-Length value");
                }
            } else {
                headers.insert(key, value);
            }
        }
    }
    if let Some(n) = content_length {
        unsafe {
            let body_ptr = body_part.as_ptr();
            let body_slice = std::slice::from_raw_parts(body_ptr, n);
            let body_str = str::from_utf8_unchecked(body_slice);
            return Ok(HTTPRequest {
                method: req_parts[0].to_string(),
                path: req_parts[1].to_string(),
                headers,
                body: body_str.to_string(),
            });
        }
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

/////////////////////////////////////////////////////////////
// Test Oracle Function for HTTP Request Parsing Consistency  //
/////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inconsistent_request() {
        // This input contains duplicate "Content-Length" headers.
        // The first header indicates a length of 10 while the second indicates 5.
        let input = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 5\r\n\r\nHelloWorld";
        // For the vulnerable version, parse_input would process the second header value and return a body trimmed to "Hello",
        // which is not the intended full request ("HelloWorld").
        // In the corrected version, the parser rejects the ambiguous input.
        match parse_input(input) {
            Ok(parsed) => {
                // Vulnerable implementation would incorrectly yield a truncated body.
                // Check if the body is exactly "HelloWorld". If not, the vulnerability is exposed.
                assert_eq!(parsed.body, "HelloWorld", "Inconsistent parsing: Expected full body, but got a truncated version.");
            }
            Err(err) => {
                // For the corrected version, the error is expected due to duplicate Content-Length.
                assert_eq!(err, "Duplicate Content-Length header", "Expected rejection of ambiguous input.");
            }
        }
    }
}
