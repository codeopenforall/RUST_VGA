fn build_response(value: &str) -> String {
    let safe_value: String = value.chars().filter(|&c| c != '\r' && c != '\n').collect();
    let line = format!("Content-Type: {}\r\n", safe_value);
    format!("HTTP/1.1 200 OK\r\n{}\r\n", line)
}

#[cfg(test)]
mod tests {
    use super::build_response;

    #[test]
    fn test_header_injection_prevention() {
        // Test input containing CRLF injection attempts.
        let injected_value = "vulnerableValue\r\nInjected-Header: injectedValue";
        let response = build_response(injected_value);
        // In the safe version, CR and LF are removed from the header value,
        // so no additional header "Injected-Header:" should be present.
        let injected_header_count = response
            .lines()
            .filter(|line| line.starts_with("Injected-Header:"))
            .count();
        // The vulnerable code would have the injected header present.
        assert_eq!(injected_header_count, 0, "Injected header found in response");
    }
}

#[cfg(test)]
mod tests {
    use super::build_response;

    #[test]
    fn test_header_injection_prevention() {
        // Test input containing CRLF injection attempts.
        let injected_value = "vulnerableValue\r\nInjected-Header: injectedValue";
        let response = build_response(injected_value);
        // In the safe version, CR and LF are removed from the header value,
        // so no additional header "Injected-Header:" should be present.
        let injected_header_count = response
            .lines()
            .filter(|line| line.starts_with("Injected-Header:"))
            .count();
        // The vulnerable code would have the injected header present.
        assert_eq!(injected_header_count, 0, "Injected header found in response");
    }
}
