use std::sync::{Arc, Mutex};
use std::thread;

/// Processes sensitive data by encrypting it before transmission.
/// Returns the encrypted data.
fn encrypt(input: &str) -> String {
    let key = b'K';
    input.bytes().map(|b| (b ^ key) as char).collect()
}

/// Simulates processing output with sensitive data.
/// For demonstration purposes, this function creates a thread that encrypts a secret
/// and sends it through a channel.
fn process_output() -> String {
    let arc = Arc::new(Mutex::new(Secret { secret: "SuperSecretPassword".to_string() }));
    let arc_clone = Arc::clone(&arc);

    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        // Encrypt the secret before sending.
        let encrypted = encrypt(&arc_clone.lock().unwrap().secret);
        tx.send(encrypted).unwrap();
    });

    rx.recv().unwrap()
}

struct Secret {
    secret: String,
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