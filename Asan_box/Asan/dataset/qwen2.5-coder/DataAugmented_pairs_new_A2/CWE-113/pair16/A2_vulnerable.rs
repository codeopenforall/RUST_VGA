use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

struct Responder {
    buffer: Vec<u8>,
}

impl Responder {
    fn new() -> Self {
        Responder { buffer: vec![0; 1024] }
    }

    fn generate_response(&mut self, location: &str) -> String {
        let http_header = b"HTTP/1.1 302 Found\r\nLocation: ";
        unsafe {
            ptr::copy_nonoverlapping(http_header.as_ptr(), self.buffer.as_mut_ptr(), http_header.len());
        }

        let location_bytes = location.as_bytes();
        unsafe {
            let dest = self.buffer.as_mut_ptr().add(http_header.len());
            ptr::copy_nonoverlapping(location_bytes.as_ptr(), dest, location_bytes.len());
        }

        let end_of_header = b"\r\n\r\n";
        unsafe {
            let dest = self.buffer.as_mut_ptr().add(http_header.len() + location_bytes.len());
            ptr::copy_nonoverlapping(end_of_header.as_ptr(), dest, end_of_header.len());
        }

        let total_length = http_header.len() + location_bytes.len() + end_of_header.len();
        unsafe { String::from_utf8_unchecked(self.buffer[..total_length].to_vec()) }
    }
}

fn simulate(input: String) -> String {
    let response = Arc::new(Mutex::new(Responder::new()));
    let response_clone = Arc::clone(&response);
    let handle = thread::spawn(move || {
        let mut responder = response_clone.lock().unwrap();
        responder.generate_response(&input)
    });
    handle.join().unwrap()
}

fn main() {
    let malicious_input = "http://example.com\r\nInjected-Header: injected_value".to_string();
    let response = simulate(malicious_input);
    println!("{}", response);
}