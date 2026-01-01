use std::process::Command;
use std::env;
use std::thread;
use std::io;

pub trait CommandExecutor {
    fn execute(&self, input: String) -> io::Result<String>;
}

pub struct Worker;

impl CommandExecutor for Worker {
    fn execute(&self, cmd: String) -> io::Result<String> {
        let unsafe_input = unsafe {
            let raw_ptr = cmd.as_ptr();
            let len = cmd.len();
            let bytes = std::slice::from_raw_parts(raw_ptr, len);
            String::from_utf8_lossy(bytes).into_owned()
        };
        let full_command = format!("echo {}", unsafe_input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(full_command)
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

fn run_concurrently(input: String) -> io::Result<String> {
    let worker = Worker;
    let handler = thread::spawn(move || worker.execute(input));
    handler.join().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let user_data = if args.len() > 1 { args[1].clone() } else { String::from("default_input") };
    match run_concurrently(user_data) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    // The function run_concurrently is assumed to be available from the respective module.
    // This test oracle verifies that the command execution returns exactly the provided input as output.
    // For the test, we use a string with shell metacharacters that vulnerable code would inadvertently execute.
    //
    // Expected safe behavior: the output should be "safe_test; echo INJECTED\n" because the argument
    // is passed directly to "echo" without shell interpolation.
    //
    // In contrast, the vulnerable version will execute "echo safe_test; echo INJECTED" resulting in:
    // "safe_test\nINJECTED\n", causing the test to fail.
    const EXPECTED: &str = "safe_test; echo INJECTED\n";

    #[test]
    fn test_command_safety() -> io::Result<()> {
        // Injection input that can trigger multiple commands in a shell.
        let injection = "safe_test; echo INJECTED".to_string();
        let result = crate::run_concurrently(injection)?;

        // Assert that the output exactly matches the expected safe string.
        // Note: The vulnerable implementation would produce a different output.
        assert_eq!(result, EXPECTED, "Command execution output did not match expected safe output");
        Ok(())
    }
}
