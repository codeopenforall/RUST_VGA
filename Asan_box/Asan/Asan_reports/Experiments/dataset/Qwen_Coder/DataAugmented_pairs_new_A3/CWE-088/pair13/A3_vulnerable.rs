use std::process::Command;

pub struct Executor {
    command: String,
}

impl Executor {
    pub fn new(input: String) -> Self {
        Executor { command: input }
    }

    pub fn run(&self) -> Result<String, String> {
        let cmd_str = format!("ls {}", self.command);
        unsafe {
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd_str)
                .output()
                .map_err(|e| e.to_string())?;
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).into_owned())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).into_owned())
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user_input = if args.len() > 1 { args[1].clone() } else { String::new() };
    let executor = Executor::new(user_input);
    match executor.run() {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("Error: {}", err),
    }
}

//////////////// Test Oracle //////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // The following tests assume that the Executor struct in each code version is available.
    // For the vulnerable build, the injected input will be processed and execute extra commands,
    // whereas in the corrected build, the input validation will cause a panic.

    // To run these tests, compile the vulnerable and corrected versions separately.
    // When running the vulnerable build, this test should fail because it detects injected output.
    // When running the corrected build, it should pass (by panicking on unsafe input).

    #[test]
    fn test_injection() {
        let malicious_input = String::from("nonexistent; echo injected");
        // Catch a panic if one occurs.
        let result = panic::catch_unwind(|| {
            // Depending on the build (vulnerable vs. corrected), this may or may not panic.
            let exec = super::Executor::new(malicious_input);
            exec.run()
        });
        match result {
            // If no panic occurred, then we check that the output does not contain injected text.
            Ok(exec_result) => {
                match exec_result {
                    Ok(output) => {
                        // In the vulnerable version, "echo injected" will cause "injected" to appear.
                        assert!(
                            !output.contains("injected"),
                            "Test failed: command injection vulnerability exploited."
                        );
                    },
                    Err(err) => {
                        // Even if error, ensure it doesn't reflect injected content.
                        assert!(
                            !err.contains("injected"),
                            "Test failed: command injection vulnerability exploited in error message."
                        );
                    }
                }
            },
            // A panic is expected in the fixed version.
            Err(_) => {
                // The fixed version panics due to input validation, so this branch is considered safe.
            }
        }
    }
}
