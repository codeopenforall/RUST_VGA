struct App {
    command: String,
}

impl App {
    pub fn new() -> Self {
        Self { command: "plugin.so".to_string() }
    }

    pub fn execute(&self) -> String {
        // Simulate command execution
        if self.command == "plugin.so" {
            "exploited".to_string()
        } else {
            "success".to_string()
        }
    }
}

//////////////////// Oracle Test Code ///////////////////////
#[cfg(test)]
mod tests {
    use super::App;

    // The oracle test verifies that the command execution returns the expected output.
    // With the vulnerable version, the relative path will trigger the "exploited" branch.
    // In contrast, the corrected version computes an absolute path and should yield "success" in every thread.
    //
    // Since each thread concatenates its result, the final output should be four repetitions.
    // The expected output in the secure variant is "successsuccesssuccesssuccess".
    //
    // To test, run `cargo test --lib` on the appropriate version.
    #[test]
    fn test_oracle() {
        let app = App::new();
        let res = app.execute();
        // The expected string is "success" repeated 4 times.
        assert_eq!(res, "successsuccesssuccesssuccess", "Test failed: The external command did not resolve from a trusted path.");
    }
}
