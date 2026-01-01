use std::thread;

struct Config {
    secret: String,
}

fn xor_cipher(data: &str, key: u8) -> String {
    data.bytes()
        .map(|b| (b ^ key) as char)
        .collect()
}

fn transmit(cfg: &Config) -> String {
    let encrypted = xor_cipher(&cfg.secret, b'K');
    encrypted
}

fn process_output() -> String {
    let cfg = Config {
        secret: String::from("supersecret"),
    };
    let handle = thread::spawn(move || {
        transmit(&cfg)
    });
    handle.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transmission_encryption() {
        let output = process_output();
        assert_ne!(output, "supersecret", "The transmitted data is not encrypted!");
    }
}