#![allow(dead_code)]
use std::sync::Arc;
use std::thread;
use std::process::Command;

struct Executor {
    binary: String,
}

impl Executor {
    fn new(binary: &str) -> Self {
        Executor { binary: binary.to_string() }
    }

    fn run(&self) -> Result<String, String> {
        unsafe {
            let result = Command::new(&self.binary).arg("--version").output();
            match result {
                Ok(output) => Ok(String::from_utf8_lossy(&output.stdout).into_owned()),
                Err(error) => Err(format!("Execution failed: {}", error)),
            }
        }
    }
}

fn main() {
    let executor = Arc::new(Executor::new("untrusted_binary"));
    let threads: Vec<_> = (0..5).map(|_| {
        let exec = Arc::clone(&executor);
        thread::spawn(move || {
            match exec.run() {
                Ok(_) => println!("Command executed successfully"),
                Err(e) => println!("Error executing command: {}", e),
            }
        })
    }).collect();

    for t in threads {
        t.join().unwrap();
    }
    println!("All threads completed execution.");
}

/*
Test Oracle Function:
This test checks that creating a new Processor with a relative executable path causes a panic.
For the fixed (corrected) code, the following unit test will pass.
For the vulnerable code, where no such check exists, the test will fail.

Note: This oracle function should be compiled and run separately from the vulnerable/fixed binaries.
*/

#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    #[should_panic(expected = "Only absolute paths are allowed")]
    fn test_absolute_path_enforcement() {
        // This should panic in the corrected version, ensuring that only absolute paths are permitted.
        let _instance = Processor::new("relative_executable");
    }
}
