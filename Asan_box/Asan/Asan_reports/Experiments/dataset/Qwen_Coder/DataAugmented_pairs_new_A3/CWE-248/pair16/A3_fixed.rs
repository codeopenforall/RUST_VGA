use std::thread;

struct Processor;

impl Processor {
    fn execute(&self) {
        let handle = thread::spawn(|| {
            unsafe {
                let mut data: i32 = 10;
                let ptr: *mut i32 = &mut data;
                *ptr = *ptr - 10;
                if *ptr == 0 {
                    println!("Avoiding division by zero due to invalid operation");
                }
            }
        });
        let _ = handle.join().unwrap();
    }
}

fn run() {
    let proc = Processor;
    proc.execute();
}

fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;

    // This test acts as an oracle. When running against the problematic version, 
    // the call to run() will cause an unwinding panic and the test will fail.
    // In the corrected code, run() recovers from the panic and the test passes.
    #[test]
    fn test_run() {
        // Execute the application logic within a catch_unwind block.
        let result = catch_unwind(|| {
            // Call the application's run function.
            crate::run();
        });
        // Assert that the execution does not result in an unhandled panic.
        assert!(result.is_ok(), "Application terminated abnormally due to unhandled panic");
    }
}
