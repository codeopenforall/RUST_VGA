fn build_response(input: &str) -> String {
    let base = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n";
    let sanitized: String = input.chars().filter(|&c| c != '\r' && c != '\n').collect();
    let combined = String::from(base) + &sanitized + "\r\n\r\n";
    combined
}