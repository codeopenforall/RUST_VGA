pub mod app {
    use std::sync::{Arc, Mutex};
    use std::thread;
    #[derive(Debug)]
    pub struct SecureData {
        pub secret: String,
        pub public_info: String,
    }
    impl SecureData {
        pub fn new(secret: &str, info: &str) -> Self {
            SecureData {
                secret: secret.to_owned(),
                public_info: info.to_owned(),
            }
        }
    }
    pub fn process(secret: &str) -> Result<String, String> {
        unsafe {
            let _pointer = secret.as_ptr();
            Err("Decryption failed due to an incorrect key".to_owned())
        }
    }
    pub fn run_app() {
        let data = Arc::new(Mutex::new(SecureData::new("SuperSecretKey", "NormalData")));
        let threads: Vec<_> = (0..3)
            .map(|_| {
                let data_clone = data.clone();
                thread::spawn(move || {
                    let secret_value = data_clone.lock().unwrap().secret.clone();
                    let result = process(&secret_value);
                    match result {
                        Ok(res) => println!("Processed result: {}", res),
                        Err(e) => panic!("Operation failed with error: {}", e),
                    }
                })
            })
            .collect();
        for handle in threads {
            handle.join().unwrap();
        }
    }
}
fn main() {
    app::run_app();
}

#[cfg(test)]
mod tests {
    // Import the module under test.
    // Ensure that the tested module is compiled either as vulnerable or fixed version.
    use crate::app::process;

    #[test]
    fn sensitive_info_not_exposed() {
        let secret = "SuperSecretKey";
        let result = process(secret);
        match result {
            Err(e) => {
                // The test is designed to ensure that error messages do not leak sensitive information.
                // In the vulnerable version, the error text would contain the secret, causing the test to fail.
                // In the sanitized version, the secret is not present.
                assert!(
                    !e.contains(secret),
                    "Sensitive information was exposed in error message."
                );
            }
            Ok(_) => panic!("Expected an error result, but got success."),
        }
    }
}
