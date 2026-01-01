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

    unsafe fn append(&self, label: &str, val: &str) {
        let composed = format!("{}: {}", label, val);
        // Remove CRLF characters from the composed string
        let sanitized = composed.replace("\r\n", "").replace("\r", "").replace("\n", "");
        let ptr = sanitized.as_ptr();
        let length = sanitized.len();
        let bytes = std::slice::from_raw_parts(ptr, length);
        let assembled = String::from_utf8_lossy(bytes).into_owned();
        let mut guard = self.data.lock().unwrap();
        guard.push(assembled);
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
