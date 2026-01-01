struct Loader {
    path: String,
}

fn run_process() -> String {
    let loader = Loader {
        path: String::from("dummy_executable"),
    };

    // Simulate process execution
    format!("{},{},{}", "Malicious execution", "Malicious execution", "Malicious execution")
}

fn main() {
    // Main function can remain empty for this example
}

#[cfg(test)]
mod tests {
    use super::run_process;

    #[test]
    fn test_execution() {
        // The expected safe execution output should be three instances of "Trusted execution" joined by commas.
        let expected = "Trusted execution,Trusted execution,Trusted execution";
        let result = run_process();
        // For the vulnerable version, run_process will return "Malicious execution,Malicious execution,Malicious execution"
        // causing this test to fail; for the fixed version, the test will pass.
        assert_eq!(result, expected, "Execution output does not match expected trusted result");
    }
}
