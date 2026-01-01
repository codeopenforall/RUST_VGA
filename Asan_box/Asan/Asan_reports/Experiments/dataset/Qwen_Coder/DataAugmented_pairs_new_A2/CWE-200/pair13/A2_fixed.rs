use std::fmt;
use std::sync::Arc;
use std::thread;

#[derive(Clone)]
struct Sensitive {
    user: String,
    key: String,
}

impl fmt::Debug for Sensitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sensitive {{ user: {}, key: \"[REDACTED]\" }}", self.user)
    }
}

fn process(s: Arc<Sensitive>) -> Result<(), String> {
    unsafe {
        let ptr = s.key.as_ptr();
        let _ = *ptr;
    }
    Err("Operation failed due to an unexpected error.".into())
}

pub fn run_app() -> String {
    let data = Arc::new(Sensitive {
        user: "alice".into(),
        key: "SuperSecretPassword".into(),
    });
    let data_cloned = Arc::clone(&data);
    let handle = thread::spawn(move || process(data_cloned));
    match handle.join() {
        Ok(Err(e)) => e,
        Ok(Ok(_)) => "Success".into(),
        Err(err) => {
            if let Some(s) = err.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "Unknown error".into()
            }
        }
    }
}

fn main() {
    let result = run_app();
    if result != "Success" {
        panic!("Fatal error: {}", result);
    }
}

//////////////// Test Oracle //////////////////
// This test oracle function is meant to be compiled and run separately from the above application code.
// It verifies that the error message returned from run_app() does NOT contain the sensitive key.
// For the vulnerable version, the test will fail because the secret "SuperSecretPassword" appears in the error message.
// For the corrected version, the test will pass.

#[cfg(test)]
mod tests {
    use super::run_app;

    #[test]
    fn test_secret_not_exposed() {
        let error_message = run_app();
        // The secret "SuperSecretPassword" should not appear in the error message.
        assert!(
            !error_message.contains("SuperSecretPassword"),
            "Sensitive information leaked in error message: {}",
            error_message
        );
    }
}
