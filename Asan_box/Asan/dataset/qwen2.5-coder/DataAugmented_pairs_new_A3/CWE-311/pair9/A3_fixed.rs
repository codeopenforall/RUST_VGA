const XOR_KEY: u8 = 0xAA;

fn xor_encrypt(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ XOR_KEY).collect()
}

fn to_hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect::<String>()
}

struct SecretHolder {
    secret: String,
}

impl SecretHolder {
    pub fn new(secret: &str) -> Self {
        SecretHolder {
            secret: secret.to_string(),
        }
    }

    pub fn transmit_secret(&self) -> String {
        let bytes = self.secret.as_bytes().to_vec();
        let encrypted = xor_encrypt(&bytes);
        to_hex(&encrypted)
    }
}

#[cfg(test)]
mod tests {
    use super::SecretHolder;

    #[test]
    fn test_encrypted_transmission() {
        let holder = SecretHolder::new("supersecret");
        let transmitted = holder.transmit_secret();
        // In the vulnerable version, the secret is sent in clear text ("supersecret").
        // In the fixed version, it is encrypted and encoded as hex.
        // This test fails if the transmitted data equals the clear text.
        assert_ne!(transmitted, "supersecret", "Sensitive data transmitted in clear text!");
    }
}