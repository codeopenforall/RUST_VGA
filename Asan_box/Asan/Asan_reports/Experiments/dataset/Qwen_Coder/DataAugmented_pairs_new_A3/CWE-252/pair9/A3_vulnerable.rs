fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: program <input>");
        return;
    }

    let input = &args[1];
    unsafe {
        let _ = critical(input);
    }

    println!("Success: 42");
}

unsafe fn critical(input: &str) -> Result<(), String> {
    if input == "150" {
        Err("Critical error".to_string())
    } else {
        Ok(())
    }
}

/*
The following test oracle is a standalone module to verify the behavior difference:
- When executed with the triggering input (e.g., "150"), the fixed version should output "Failure"
  because the critical check propagates an error.
- The vulnerable version, in contrast, disregards the error and outputs "Success: 42".
This oracle uses a command invocation to run the compiled binary and asserts on its output.
*/

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;

    #[test]
    fn oracle() {
        // Adjust the binary name as needed. Here it is assumed to be "program".
        // The test will run the binary with input "150" (triggering an error per design).
        let output = Command::new("./target/debug/program")
            .arg("150")
            .output()
            .expect("Failed to execute process");

        let stdout = str::from_utf8(&output.stdout).expect("Output not UTF-8");
        // The proper (fixed) behavior: critical check should catch the error.
        assert!(
            stdout.contains("Failure"),
            "Test failed: expected a failure output, got: {}",
            stdout
        );
    }
}
