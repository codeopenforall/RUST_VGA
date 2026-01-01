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