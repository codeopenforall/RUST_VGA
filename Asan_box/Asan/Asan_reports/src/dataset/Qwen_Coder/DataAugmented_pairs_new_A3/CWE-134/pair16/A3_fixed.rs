use std::sync::{Arc, Mutex};
use std::thread;
struct LogContainer {
    secret: &'static str,
}
impl LogContainer {
    fn new() -> Self {
        LogContainer { secret: "VerySecretInfo" }
    }
    fn process(&self, user_data: &str) -> String {
        format!("User message: {}", user_data)
    }
}
fn simulate(user_data: &str) -> String {
    let container = Arc::new(LogContainer::new());
    let output = Arc::new(Mutex::new(String::new()));
    let container_clone = Arc::clone(&container);
    let output_clone = Arc::clone(&output);
    let handle = thread::spawn(move || {
        let result = container_clone.process(user_data);
        let mut out_lock = output_clone.lock().unwrap();
        *out_lock = result;
    });
    handle.join().unwrap();
    let res = output.lock().unwrap();
    res.clone()
}
fn main() {
    let safe_input = "User provided data";
    let result = simulate(safe_input);
    println!("Result: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle validates the behavior under malicious input.
    // It expects that the returned string DOES NOT contain the sensitive secret.
    // In the vulnerable version, using malicious input like "Alert: {}" would include the secret,
    // causing this test to fail. The fixed version avoids revealing the secret.
    #[test]
    fn test_format_string_handling() {
        let malicious_input = "Alert: {}";
        let secret = "VerySecretInfo";

        // Call the simulate function from the code under test.
        let output = simulate(malicious_input);

        // The oracle asserts that the secret is not present in the output.
        // For a secure implementation, the secret should not have been formatted into the output.
        assert!(
            !output.contains(secret),
            "Sensitive information was leaked in the output!"
        );
    }
}
