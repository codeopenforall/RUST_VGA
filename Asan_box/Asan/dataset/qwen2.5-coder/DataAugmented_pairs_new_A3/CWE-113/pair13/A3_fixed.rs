struct Responder;

impl Responder {
    fn sanitize(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }

    pub fn send_response(input: &str) -> String {
        let base = "HTTP/1.1 200 OK\r\n";
        let safe_input = Self::sanitize(input);
        let header = format!("{}{}{}", base, safe_input, "\r\nContent-Length: 0\r\n\r\n");
        header
    }
}