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