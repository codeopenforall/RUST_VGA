use std::thread;

pub struct Config {
    pub secret: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            secret: "supersecret".to_string(),
        }
    }
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

pub fn process_output() -> String {
    let cfg = Config::new();
    let handle = thread::spawn(move || {
        transmit(&cfg)
    });
    handle.join().unwrap()
}

fn main() {
    let output = process_output();
    println!("Transmitted data: {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    // This oracle test expects that the transmitted output does not match
    // the raw secret ("supersecret"). In the vulnerable version, the output
    // is in clear text and equals "supersecret", so the test should fail.
    // In the fixed version, the output is encrypted and must not equal "supersecret".
    #[test]
    fn test_transmission_encryption() {
        let output = process_output();
        // Direct comparison with the cleartext secret.
        assert_ne!(output, "supersecret", "The transmitted data is not encrypted!");
    }
}
