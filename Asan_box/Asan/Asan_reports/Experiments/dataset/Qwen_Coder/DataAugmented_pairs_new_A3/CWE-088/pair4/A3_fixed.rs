use std::process::Command;

fn run_app() -> String {
    let untrusted = "echo test; echo injected";
    safe_run(untrusted)
}

fn safe_run(untrusted: &str) -> String {
    let clean: String = untrusted
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();
    let output = Command::new("echo")
        .arg(clean)
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/*
   Test Oracle:
   The following test function is used to assert that the application behaves correctly.
   The expected output is the sanitized result without any injected commands.
   For the insecure version, the test should fail as it produces additional output stemming from the injection.
   For the secure version, the test should pass.
*/
#[cfg(test)]
mod tests {
    // Note: Adjust the import path if needed based on your module organization.
    use super::run_app;

    #[test]
    fn injection_test() {
        // Execute the application and capture its output.
        let result = run_app();
        // The secure flow should output a single echoed line that does not include injected commands.
        // Given the untrusted string "echo test; echo injected" and the sanitization process,
        // the expected output filters out the semicolon and results in "echo test echo injected\n".
        let expected = "echo test echo injected\n";
        assert_eq!(result, expected, "The output does not match the expected safe output.");
    }
}
