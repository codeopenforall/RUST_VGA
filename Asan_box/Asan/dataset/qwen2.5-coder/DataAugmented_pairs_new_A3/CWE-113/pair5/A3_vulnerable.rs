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
        let header = format!("HTTP/1.1 200 OK\r\nCustom-Header: {}\r\n", self.input);
        header
    }
}