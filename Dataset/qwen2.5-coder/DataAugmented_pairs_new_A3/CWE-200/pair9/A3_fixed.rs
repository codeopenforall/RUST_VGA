// Define the module structure as expected by the tests.
pub mod app {
    // Define the public function that will be tested.
    pub fn process(secret: &str) -> Result<(), String> {
        // Simulate a decryption failure scenario.
        // In the fixed version, we avoid exposing the secret in error messages.
        Err("Decryption failed due to an incorrect key".to_owned())
    }
}