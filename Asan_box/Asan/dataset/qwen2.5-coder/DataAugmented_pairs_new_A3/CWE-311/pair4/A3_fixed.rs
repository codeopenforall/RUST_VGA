use std::sync::{Arc, Mutex};

struct Config {
    api_key: String,
}

impl Config {
    fn new(api_key: &str) -> Self {
        Config {
            api_key: api_key.to_string(),
        }
    }

    fn transmit(&self) -> Vec<u8> {
        const XOR_KEY: u8 = 0xAA;
        fn xor_cipher(data: &[u8]) -> Vec<u8> {
            data.iter().map(|b| b ^ XOR_KEY).collect()
        }

        let plaintext = self.api_key.as_bytes();
        let encrypted_result = xor_cipher(plaintext);

        let shared = Arc::new(Mutex::new(encrypted_result));
        let encrypted_result = shared.lock().unwrap();
        println!("Transmitted encrypted data: {:?}", encrypted_result);

        encrypted_result.clone()
    }
}