use std::thread;
use std::ptr;
struct Response {
    headers: String,
    body: String,
}
fn create_response(user_input: &str, body: &str) -> Response {
    let mut header_buf = [0u8; 256];
    let len = user_input.len();
    if len >= header_buf.len() {
        panic!("Input too long");
    }
    unsafe {
        let dst = header_buf.as_mut_ptr();
        let src = user_input.as_ptr();
        ptr::copy_nonoverlapping(src, dst, len);
    }
    let header_str = String::from_utf8_lossy(&header_buf[..len]).into_owned();
    // Sanitize the header to remove CRLF sequences
    let sanitized_header = header_str.replace("\r\n", "");
    let headers = format!("X-Custom-Header: {}\r\n", sanitized_header);
    Response { headers, body: body.to_string() }
}
fn main() {
    let user_input = "NormalValue\r\nInjected: malicious";
    let body = "<html><body>Test</body></html>";
    let handle = thread::spawn(move || {
        create_response(user_input, body)
    });
    let res = handle.join().unwrap();
    println!("Headers:\n{}", res.headers);
    println!("Body:\n{}", res.body);
}

/*
The following test oracle function is intended to be compiled separately from the main source.
It tests the HTTP header generation routine by supplying an input that includes CRLF sequences.
For the vulnerable version, the test will fail because the resulting header contains the injected line,
whereas in the fixed version, the injection is removed.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_header_sanitization() {
        let malicious_input = "SafeValue\r\nInjected: evil";
        let resp = create_response(malicious_input, "Test Body");
        // The fixed version should not contain the injected header part.
        assert!(!resp.headers.contains("\r\nInjected:"), "CRLF injection detected in HTTP header!");
    }
}
