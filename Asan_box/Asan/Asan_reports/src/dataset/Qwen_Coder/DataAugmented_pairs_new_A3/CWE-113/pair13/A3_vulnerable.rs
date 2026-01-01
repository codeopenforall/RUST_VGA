pub struct Responder;

impl Responder {
    pub fn send_response(input: &str) -> String {
        let base = "HTTP/1.1 200 OK\r\n";
        let header = format!("{}{}{}", base, input, "\r\nContent-Length: 0\r\n\r\n");
        header
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
