use std::sync::{Arc, Mutex};
use std::thread;

fn critical(input: u32) -> Result<(), &'static str> {
    if input < 100 {
        Ok(())
    } else {
        Err("Input too high")
    }
}

fn execute(input: u32) -> Result<u32, &'static str> {
    let outcome = Arc::new(Mutex::new(0));
    let outcome_thread = Arc::clone(&outcome);
    let h = thread::spawn(move || {
        unsafe {
            let _ = critical(input); // CWE-252: Ignoring the Result
        }
        let mut val = outcome_thread.lock().unwrap();
        *val = 42;
    });
    h.join().unwrap();
    Ok(*outcome.lock().unwrap())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_val: u32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    match execute(input_val) {
        Ok(v) => println!("Success: {}", v),
        Err(e) => println!("Failure: {}", e),
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
