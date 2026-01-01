use std::sync::{Arc, Mutex};
use std::thread;

struct Server {
    data: Arc<Mutex<Vec<String>>>,
}

impl Server {
    fn initiate() -> Self {
        Server {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn cleanse(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }

    unsafe fn append(&self, label: &str, val: &str) {
        let purified = Server::cleanse(val);
        let composed = format!("{}: {}", label, purified);
        let mut guard = self.data.lock().unwrap();
        guard.push(composed);
    }

    fn retrieve(&self) -> Vec<String> {
        let guard = self.data.lock().unwrap();
        guard.clone()
    }
}

fn main() {
    let attacker_input = "Value\r\nInjected-Header: injected";
    let server = Server::initiate();
    let shared = server.data.clone();
    let handler = thread::spawn(move || {
        unsafe {
            let srv = Server { data: shared };
            srv.append("X-Test", attacker_input);
        }
    });
    handler.join().unwrap();
    let headers = server.retrieve();
    for line in headers {
        println!("{}", line);
    }
}

///////////////////////////////////////////////////////////////////////
// Test Oracle: This unit test fails when CRLF injection is present and
// passes when the header value is correctly sanitized.
// This test should be placed in a separate test module.
///////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_integrity() {
        // Input containing CRLF attempting to inject an extra header.
        let attacker_input = "Value\r\nInjected-Header: injected";
        let server = Server::initiate();
        
        // Directly call the unsafe method.
        unsafe {
            server.append("X-Test", attacker_input);
        }
        
        let headers = server.retrieve();
        // The test asserts that none of the headers contain CR or LF characters.
        for header in headers {
            assert!(!header.contains('\r'), "Header contains CR");
            assert!(!header.contains('\n'), "Header contains LF");
        }
    }
}
