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