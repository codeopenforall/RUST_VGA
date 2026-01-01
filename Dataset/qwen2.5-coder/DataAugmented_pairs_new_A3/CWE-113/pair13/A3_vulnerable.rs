pub struct Responder;

impl Responder {
    pub fn send_response(input: &str) -> String {
        let base = "HTTP/1.1 200 OK\r\n";
        let header = format!("{}{}{}", base, input, "\r\nContent-Length: 0\r\n\r\n");
        header
    }
}