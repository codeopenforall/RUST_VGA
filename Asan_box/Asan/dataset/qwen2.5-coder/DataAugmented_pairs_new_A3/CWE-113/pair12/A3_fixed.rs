use std::sync::{Arc, Mutex};
use std::thread;

struct HttpResponse {
    header: String,
}

impl HttpResponse {
    pub fn new(input: &str) -> HttpResponse {
        let sanitized_input = input.replace('\r', "").replace('\n', "");
        HttpResponse {
            header: format!("Location: {}", sanitized_input),
        }
    }

    pub fn send(&self) -> String {
        self.header.clone()
    }
}

fn main() {
    let input = "http://example.com\r\nSet-Cookie: sessionid=bad";
    let response = Arc::new(Mutex::new(HttpResponse::new(input)));
    let mut threads = vec![];
    for _ in 0..4 {
        let clone_response = Arc::clone(&response);
        threads.push(thread::spawn(move || {
            let resp = clone_response.lock().unwrap();
            let header = resp.send();
            println!("{}", header);
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
}