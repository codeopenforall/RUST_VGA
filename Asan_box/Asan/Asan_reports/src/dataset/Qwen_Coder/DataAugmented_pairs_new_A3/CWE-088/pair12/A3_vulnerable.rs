use std::process::Command;
use std::sync::Arc;
use std::thread;
struct Executor {
    data: Arc<String>,
}
impl Executor {
    fn run(&self) -> Result<String, &'static str> {
        let input = self.data.as_str();
        // Remove the check for forbidden characters
        // if input.contains(';') || input.contains('&') || input.contains('|') {
        //     return Err("input contains forbidden characters");
        // }
        let output = Command::new("echo")
            .arg(input)
            .output()
            .map_err(|_| "failed to execute command")?;
        if !output.status.success() {
            return Err("command did not return success");
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
fn main() {
    let input = "hello; echo injected";
    let exec = Executor {
        data: Arc::new(input.to_string()),
    };
    let handle = thread::spawn(move || {
        match exec.run() {
            Ok(result) => println!("Output: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod test_oracle {
    use std::sync::Arc;
    // Assume that the Executor struct from the respective version is imported here.
    // The test should be compiled separately against the built binary (vulnerable or fixed).

    // The injection_test function verifies that the provided input causing potential injection is properly handled.
    // In the vulnerable version, the command injection will succeed and produce unexpected output, causing the test to fail.
    // In the fixed version, input validation will reject the input and return an error, and the test will pass.
    #[test]
    fn injection_test() {
        // This test uses input that contains shell injection characters.
        let input = "hello; echo injected";
        let exec = super::Executor {
            data: Arc::new(input.to_string()),
        };

        let res = exec.run();

        // In a secure implementation, the function should detect the forbidden characters and return an error.
        // Thus, we assert that the result is an error.
        assert!(res.is_err(), "The input was not properly validated against injection attacks");
    }
}
