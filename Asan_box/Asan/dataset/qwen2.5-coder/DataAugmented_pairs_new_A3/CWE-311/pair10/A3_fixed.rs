struct Config {
    data: String,
}

impl Config {
    fn new(secret: &str) -> Self {
        Config {
            data: secret.to_string(),
        }
    }

    fn xor_transform(input: &str, key: u8) -> String {
        let transformed: Vec<u8> = input.bytes().map(|b| b ^ key).collect();
        transformed.iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn send(&self) -> String {
        let key: u8 = 0xAA;
        Self::xor_transform(&self.data, key)
    }
}