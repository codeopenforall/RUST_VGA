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

/////////////////////// Test Oracle Function ///////////////////////
// This test oracle function is designed to be compiled separately
// from the above implementations. It should be run against both the
// insecure and secure builds. On the insecure build, the test will fail,
// whereas on the secure build, the test will pass.

#[cfg(test)]
mod tests {
    // Import the Responder from the implementation under test.
    // In practice, one would compile and test the insecure and secure variants separately.
    use super::Responder;

    #[test]
    fn oracle() {
        // Input crafted to trigger CRLF injection if unsanitized.
        let malicious = "malicious\r\nInjected: yes";
        let response = Responder::send_response(malicious);
        // Check for the presence of the injected header marker.
        // In the vulnerable variant, the response will contain an extra header line "Injected: yes".
        // The secure variant removes CRLF sequences so that "Injected:" never appears as a separate header.
        assert!(
            !response.contains("\r\nInjected:"),
            "CRLF injection vulnerability triggered"
        );
    }
}
