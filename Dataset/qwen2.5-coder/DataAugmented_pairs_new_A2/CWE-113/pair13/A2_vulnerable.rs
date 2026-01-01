use std::ptr;
use std::thread;

struct Responder;

impl Responder {
    fn construct_header(input: &str) -> String {
        let base = "HTTP/1.1 302 Found\r\nLocation: ";
        let header = format!("{}{}{}", base, input, "\r\nContent-Length: 0\r\n\r\n");
        header
    }

    fn dispatch_response(input: &str) -> String {
        let header = Self::construct_header(input);
        let length = header.len();
        let mut buffer = Vec::with_capacity(length);
        unsafe {
            buffer.set_len(length);
            ptr::copy_nonoverlapping(header.as_ptr(), buffer.as_mut_ptr(), length);
        }
        String::from_utf8(buffer).unwrap()
    }
}

fn main() {
    let tasks: Vec<_> = (0..4).map(|_| {
        thread::spawn(|| {
            let input = std::env::args().nth(1).unwrap_or(String::from("default"));
            let response = Responder::dispatch_response(&input);
            println!("{}", response);
        })
    }).collect();
    for t in tasks {
        t.join().unwrap();
    }
}