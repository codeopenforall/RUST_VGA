struct Config {
    data: String,
}

impl Config {
    pub fn new(secret: &str) -> Self {
        Config {
            data: secret.to_string(),
        }
    }

    pub fn send(&self) -> String {
        unsafe {
            let raw_ptr = self.data.as_ptr();
            let len = self.data.len();
            let bytes = std::slice::from_raw_parts(raw_ptr, len);
            String::from_utf8_lossy(bytes).to_string()
        }
    }
}