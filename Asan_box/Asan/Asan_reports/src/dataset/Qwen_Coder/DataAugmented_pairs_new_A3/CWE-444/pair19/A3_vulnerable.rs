use std::str;

struct HttpRequest;

impl HttpRequest {
    unsafe fn parse_body(raw: *const u8, len: usize) -> String {
        let slice = std::slice::from_raw_parts(raw, len);
        String::from_utf8_lossy(slice).into_owned()
    }
}

fn process_request(input: &str) -> Result<String, &'static str> {
    let lines: Vec<&str> = input.lines().collect();
    let mut transfer_encoding = false;
    let mut content_length = None;
    let mut body_str = "";

    for line in lines.iter() {
        if line.starts_with("Transfer-Encoding:") {
            transfer_encoding = true;
        } else if line.starts_with("Content-Length:") {
            if let Some(len) = line.split(':').nth(1) {
                if let Ok(num) = len.trim().parse::<usize>() {
                    content_length = Some(num);
                }
            }
        } else if !line.is_empty() {
            body_str = line;
            break;
        }
    }

    let mut body = String::new();

    if transfer_encoding && content_length.is_some() {
        unsafe {
            let ptr = body_str.as_ptr().offset(1); 
            let len = content_length.unwrap();
            body = HttpRequest::parse_body(ptr, len);
        }
    } else if transfer_encoding {
        // Handle chunked encoding
    }

    Ok(body)
}

#[cfg(test)]
mod tests {
    // The test oracle applies the same HTTP input to the parser.
    // In the vulnerable variant, the unsafe offset leads to extracting "elloW" instead of "HelloWorld".
    // The corrected version should consistently return "HelloWorld".
    use super::*;

    #[test]
    fn test_http_request_parsing() {
        let input = "POST /test HTTP/1.1\r\nTransfer-Encoding: chunked\r\nContent-Length: 5\r\n\r\nHelloWorld";
        let result = process_request(input).expect("Processing failed");
        // Expect the full "HelloWorld" as the correct interpretation.
        assert_eq!(result, "HelloWorld", "HTTP request parsing did not prioritize headers correctly");
    }
}
