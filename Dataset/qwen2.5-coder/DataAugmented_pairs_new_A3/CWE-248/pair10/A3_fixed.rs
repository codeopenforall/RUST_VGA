use std::panic;
use std::thread;

struct Processor;

impl Processor {
    unsafe fn execute(&self, divisor: i32) -> i32 {
        if divisor == 0 {
            panic!("Division by zero");
        }
        10 / divisor
    }
}

static mut PROC: Processor = Processor;

fn perform_action(divisor: i32) -> Result<i32, &'static str> {
    let result = panic::catch_unwind(|| {
        unsafe { PROC.execute(divisor) }
    });
    match result {
        Ok(val) => Ok(val),
        Err(_) => Err("Operation failed: division by zero or invalid input"),
    }
}

fn main() {
    let handle = thread::spawn(|| {
        match perform_action(0) {
            Ok(result) => println!("Computed result: {}", result),
            Err(e) => println!("Error encountered: {}", e),
        }
    });

    handle.join().expect("Thread panicked unexpectedly");
}