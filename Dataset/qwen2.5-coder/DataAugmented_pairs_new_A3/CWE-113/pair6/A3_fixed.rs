fn assembled_response(input: &str) -> String {
    let safe_input: String = input.chars().filter(|&c| c != '\r' && c != '\n').collect();
    let mut header = String::new();
    header.push_str("Content-Length: ");
    header.push_str(&safe_input.len().to_string());
    header.push_str("\r\n\r\n");
    header.push_str(&safe_input);
    header
}