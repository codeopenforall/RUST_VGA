use std::{thread, panic};

fn trigger() {
    // Simulate a function that might cause a panic
    panic!("Triggered a panic");
}

fn run_app() -> Result<(), &'static str> {
    let handle = thread::spawn(|| {
        let result = panic::catch_unwind(|| {
            unsafe { trigger(); }
        });
        if result.is_err() {
            Err("Panic occurred in thread")
        } else {
            Ok(())
        }
    });

    handle.join().map_err(|_| "Thread join failed")?;
    Ok(())
}

fn main() {
    // Main function for demonstration purposes
    if let Err(e) = run_app() {
        eprintln!("Error: {}", e);
    }
}

fn test_oracle() {
    // The test oracle validates that run_app() completes without propagating a panic.
    // In the vulnerable version, run_app() would panic causing this test to fail.
    // In the fixed version, run_app() returns Ok, and the test passes.
    match run_app() {
        Ok(_) => assert!(true, "Application terminated normally"),
        Err(e) => panic!("Test failed: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_run_app() {
        test_oracle();
    }
}
