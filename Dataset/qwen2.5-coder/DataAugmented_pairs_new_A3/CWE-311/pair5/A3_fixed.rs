struct Config {
    secret: String,
}

static mut SENT_DATA: Option<String> = None;

fn encrypt_secret(data: &str, key: u8) -> String {
    data.bytes()
        .map(|b| b ^ key)
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

fn simulate_send() -> String {
    let cfg = Config {
        secret: "supersecret".to_string(),
    };
    let key: u8 = 0xAA;
    let encrypted = encrypt_secret(&cfg.secret, key);
    unsafe {
        SENT_DATA = Some(encrypted.clone());
    }
    encrypted
}

#[cfg(test)]
mod tests {
    use super::simulate_send;
    
    #[test]
    fn test_secret_encryption() {
        let transmitted = simulate_send();
        assert_ne!(
            transmitted, 
            "supersecret", 
            "Sensitive data is transmitted in clear text!"
        );
    }
}