fn sanitize(input: &str) -> String {
    input.chars().filter(|&c| c != '\r' && c != '\n').collect()
}

pub fn build_response(untrusted: &str) -> String {
    let prefix = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n";
    let safe_input = sanitize(untrusted);
    format!("{}{}", prefix, safe_input)
}

#[cfg(test)]
mod tests {
    use super::build_response;

    #[test]
    fn oracle() {
        // Test input containing CRLF injection attempt.
        let input = "good\r\nInjected: evil";
        let header = build_response(input);
        // The safe version should not contain any carriage return or newline.
        assert!(
            !header.contains('\r'),
            "Header contains a carriage return, indicating a vulnerability."
        );
        assert!(
            !header.contains('\n'),
            "Header contains a newline, indicating a vulnerability."
        );
    }
}

//////////////////////////////////////////////
// Test Oracle: This function serves as a unit test to verify that
// the header built does not contain CR or LF characters.
// For the vulnerable version, the test will fail, and for the fixed version, the test will pass.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::build_response;

    #[test]
    fn oracle() {
        // Test input containing CRLF injection attempt.
        let input = "good\r\nInjected: evil";
        let header = build_response(input);
        // The safe version should not contain any carriage return or newline.
        assert!(
            !header.contains('\r'),
            "Header contains a carriage return, indicating a vulnerability."
        );
        assert!(
            !header.contains('\n'),
            "Header contains a newline, indicating a vulnerability."
        );
    }
}
