// Define the module structure as expected by the tests.
pub mod app {
    // Define the public function that will be tested.
    pub fn process(secret: &str) -> Result<(), String> {
        // Simulate a decryption failure scenario.
        // In the fixed version, we avoid exposing the secret in error messages.
        Err("Decryption failed due to an incorrect key".to_owned())
    }
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
