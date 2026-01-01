use std::thread;

struct Config {
    secret: String,
}

fn transmit(cfg: &Config) -> String {
    let data: String;
    unsafe {
        let ptr = cfg.secret.as_ptr();
        let len = cfg.secret.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        data = String::from_utf8_unchecked(slice.to_vec());
    }
    data  
}

fn process_output() -> String {
    let cfg = Config {
        secret: "supersecret".to_string(),
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