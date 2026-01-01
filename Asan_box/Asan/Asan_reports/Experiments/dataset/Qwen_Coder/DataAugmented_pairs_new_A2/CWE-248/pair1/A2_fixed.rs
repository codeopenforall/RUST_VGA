use std::{thread, panic};

fn run_app() -> Result<(), &'static str> {
    let handle = thread::spawn(|| {
        let result = panic::catch_unwind(|| {
            unsafe { trigger(); }
        });
        if result.is_err() {
            eprintln!("Panic caught in thread");
        }
    });
    handle.join().map_err(|_| "Thread join failed")?;
    Ok(())
}

unsafe fn trigger() {
    let ptr: *const i32 = std::ptr::null();
    if ptr.is_null() {
        panic!("Simulated panic: null pointer dereference");
    }
}

fn main() {
    run_app().expect("Application encountered an error");
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
