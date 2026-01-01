use std::{thread};

unsafe fn trigger() {
    // Simulate a function that might cause a panic
    panic!("Triggered a panic");
}

fn run_app() -> Result<(), String> {
    // Intentionally propagate a panic
    unsafe { trigger(); }
    Ok(())
}

fn main() {
    let handle = thread::spawn(|| {
        run_app().unwrap();
    });

    handle.join().map_err(|_| "Thread panicked").unwrap();
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
