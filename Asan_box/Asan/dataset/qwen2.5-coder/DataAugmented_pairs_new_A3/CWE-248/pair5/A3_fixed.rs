use std::{thread, panic};

struct Executor;

impl Executor {
    fn run(&self) -> Result<(), &'static str> {
        let result = panic::catch_unwind(|| unsafe {
            // Simulate some unsafe operation that might panic
            panic!("Unsafe operation failed");
        });
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Recovered from unsafe panic"),
        }
    }
}

fn run_app() {
    let exec = Executor;
    if let Err(e) = exec.run() {
        eprintln!("Error: {}", e);
    }
}

fn main() {
    let handle = thread::spawn(|| {
        run_app();
    });

    if let Err(e) = handle.join() {
        eprintln!("Thread join error: {:?}", e);
    }
}