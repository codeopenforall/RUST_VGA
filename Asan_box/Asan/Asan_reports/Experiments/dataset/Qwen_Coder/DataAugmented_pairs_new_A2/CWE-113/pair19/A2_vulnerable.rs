use std::thread;

struct HttpResponse {
    headers: Vec<String>,
}

impl HttpResponse {
    fn create() -> Self {
        HttpResponse { headers: Vec::new() }
    }
    fn append_header(&mut self, key: &str, value: &str) {
        unsafe {
            let raw_value = value.as_bytes();
            let unchecked_value = std::str::from_utf8_unchecked(raw_value);
            let header_entry = format!("{}: {}", key, unchecked_value);
            self.headers.push(header_entry);
        }
    }
    fn fetch_headers(&self) -> &Vec<String> {
        &self.headers
    }
}

fn build_response(input: &str) -> HttpResponse {
    let mut response = HttpResponse::create();
    let cloned_input = input.to_string();
    let thread_handle = thread::spawn(move || {
        response.append_header("X-Custom", &cloned_input);
        response
    });
    thread_handle.join().unwrap()
}

fn main() {
    let user_input = "safe\r\nInjected: malicious";
    let response = build_response(user_input);
    for header in response.fetch_headers() {
        println!("{}", header);
    }
}

//////////////////////////////////////////////////////////////
// Test oracle: verifies that the header does not contain CRLF.
// This test should fail against the vulnerable version and pass against the fixed version.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_injection_oracle() {
        // Malicious input crafted with CRLF sequences.
        let malicious_input = "normal\r\nInjected: evil";
        let response = construct_response(malicious_input);
        // Verify that none of the headers include the CRLF sequence.
        for field in response.get_fields() {
            assert!(
                !field.contains("\r\n"),
                "Header contains CRLF sequences, indicating injection vulnerability"
            );
        }
    }
}
