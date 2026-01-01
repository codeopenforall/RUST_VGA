use std::panic;
use std::thread;

struct Service;

impl Service {
    fn execute(&self, trigger: bool) -> Result<(), &'static str> {
        let handle = thread::spawn(move || {
            let result = panic::catch_unwind(|| {
                if trigger {
                    panic!("Triggered panic");
                }
            });
            result
        });

        match handle.join() {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(_)) => Err("Thread encountered a panic"),
            Err(_) => Err("Failed to join thread"),
        }
    }
}

fn main() {
    let service = Service;
    match service.execute(true) {
        Ok(_) => println!("Execution completed successfully."),
        Err(e) => println!("Execution error handled: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    // The test oracle calls the main entry point.
    // In the vulnerable version, a panic will occur causing the test to fail.
    // In the fixed version, the panic is caught, and the application exits normally.
    #[test]
    fn test_entry() {
        // Capture any panic from main.
        let result = panic::catch_unwind(|| {
            // Call the entry point; this should not panic in the fixed version.
            crate::main();
        });
        // The test asserts that main did not panic.
        assert!(result.is_ok(), "Expected normal termination, but a panic occurred.");
    }
}
