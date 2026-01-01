use std::process::Command;
use std::sync::Arc;
use std::thread;

fn execute_command(input: &str) -> std::io::Result<String> {
    let cmd_line = format!("echo {}", input);
    unsafe {
        let c_str = std::ffi::CString::new(cmd_line).unwrap();
        Command::new("sh")
            .arg("-c")
            .arg(c_str.to_str().unwrap())
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

fn main() {
    let input = std::env::args().nth(1).expect("Expected input argument");
    let shared_input = Arc::new(input);
    let mut handles = vec![];
    for _ in 0..4 {
        let cloned_input = Arc::clone(&shared_input);
        handles.push(thread::spawn(move || {
            match execute_command(&cloned_input) {
                Ok(result) => result,
                Err(e) => format!("Error: {}", e),
            }
        }));
    }
    for handle in handles {
        println!("{}", handle.join().unwrap());
    }
}

//////////////////////////////////////////////////////////////
// Test Oracle Function for OS Command Injection Vulnerability
//
// This test checks that when provided with an injection string, the output from the command
// execution function equals the expected safe behavior only in the corrected code.
// In the vulnerable version, the injected shell metacharacter would be interpreted, resulting in
// additional or modified output, causing the test to fail.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::execute_command;

    #[test]
    fn test_injection_prevention() {
        // This input attempts to inject an extra command ("echo hacked").
        let injection_input = "hello; echo hacked";
        // Expected output for a safe implementation:
        // The echo command should treat the entire input as a literal argument.
        let expected_output = "hello; echo hacked\n";
        
        let output = execute_command(injection_input)
            .expect("Failed to execute command");
        
        // The assertion will fail for the vulnerable version if the shell executes the injected command.
        assert_eq!(
            output, expected_output,
            "Test failed: Detected command injection vulnerability"
        );
    }
}
