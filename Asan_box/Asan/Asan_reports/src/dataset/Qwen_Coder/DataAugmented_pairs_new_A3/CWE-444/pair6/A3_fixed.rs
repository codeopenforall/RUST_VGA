use std::collections::HashMap;

#[derive(Debug)]
struct HTTPRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

fn parse_input(input: &str) -> Result<HTTPRequest, &'static str> {
    let parts: Vec<&str> = input.splitn(2, "\r\n\r\n").collect();
    if parts.len() != 2 {
        return Err("Invalid input format");
    }

    let header_part = parts[0];
    let body_part = parts[1];

    let mut headers = HashMap::new();
    let mut header_lines = header_part.lines();
    let request_line = header_lines.next().ok_or("Missing request line")?;
    let req_parts: Vec<&str> = request_line.split_whitespace().collect();
    if req_parts.len() != 3 {
        return Err("Invalid request line");
    }

    for line in header_lines {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            continue;
        }
        let key = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();
        if headers.contains_key(&key) {
            if key == "Content-Length" {
                return Err("Duplicate Content-Length header");
            }
        }
        headers.insert(key, value);
    }

    let cl = headers.get("Content-Length").ok_or("Missing Content-Length header")?;
    let expected = cl.parse::<usize>().map_err(|_| "Invalid Content-Length value")?;
    if expected != body_part.len() {
        return Err("Message body length does not match Content-Length header");
    }

    Ok(HTTPRequest {
        method: req_parts[0].to_string(),
        path: req_parts[1].to_string(),
        headers,
        body: body_part.to_string(),
    })
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
