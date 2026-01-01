struct Server {
    input: String,
}

impl Server {
    pub fn new(input: &str) -> Self {
        Server {
            input: input.to_string(),
        }
    }

    pub fn send(&self) -> String {
        let sanitized: String = self.input.chars()
            .filter(|&c| c != '\r' && c != '\n')
            .collect();
        let header = format!("HTTP/1.1 200 OK\r\nCustom-Header: {}\r\n", sanitized);
        header
    }
}