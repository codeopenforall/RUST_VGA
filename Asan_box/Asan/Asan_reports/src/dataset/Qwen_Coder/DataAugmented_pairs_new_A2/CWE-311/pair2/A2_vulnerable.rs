use std::sync::{Arc, mpsc};
use std::thread;

struct Config {
    secret: String,
}

pub fn process_output() -> String {
    let secret = String::from("SuperSecretPassword");
    let config = Config { secret };
    let arc_config = Arc::new(config);
    let arc_clone = arc_config.clone();
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        unsafe {
            let raw: *const String = &arc_clone.secret;
            let secret_ref: &String = &*raw;
            tx.send(secret_ref.clone()).unwrap();
        }
    });
    handle.join().unwrap();
    rx.recv().unwrap()
}

fn main() {
    let data = process_output();
    println!("Transmitted: {}", data);
}

#[cfg(test)]
mod tests {
    use super::process_output;
    
    #[test]
    fn test_transmission_security() {
        // The original plaintext secret.
        let plaintext = "SuperSecretPassword";
        let output = process_output();
        
        // For a secure implementation, the output must differ from the plaintext.
        // In an insecure (vulnerable) scenario the test will fail.
        assert_ne!(output, plaintext, "Sensitive data was transmitted in cleartext.");

        // For demonstration purposes, apply the same XOR encryption on the plaintext.
        let key = b'K';
        let expected: String = plaintext.bytes().map(|b| (b ^ key) as char).collect();
        assert_eq!(output, expected, "The encryption output does not match the expected result.");
    }
}
