use std::panic;
use std::thread;

struct Processor {}

trait Execute {
    fn run(&self);
}

impl Execute for Processor {
    fn run(&self) {
        let handler = thread::spawn(|| {
            let _ = panic::catch_unwind(|| {
                unsafe {
                    panic!("Error: Unexpected panic in unsafe operation");
                }
            });
        });
        handler.join().unwrap();
    }
}

fn main() {
    let unit = Processor {};
    unit.run();
    println!("Processing completed safely (fixed version).");
}

//////////////////////////////////////////////////////////////
// The function below is a test oracle that can be used to
// verify the behavior of the module. It creates a Processor
// and calls its "run" method inside a catch_unwind. For the
// vulnerable version, this test will fail since the panic is
// not caught within the processing thread (resulting in an Err),
// whereas for the fixed version the panic is caught internally,
// and the test passes.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_processor_execution() {
        // The test catches any panic that may occur during execution.
        let result = panic::catch_unwind(|| {
            let unit = Processor {};
            unit.run();
        });
        // For the fixed version, run() completes without panicking (result.is_ok() == true).
        // For the vulnerable version, run() panics (result.is_ok() == false).
        assert!(result.is_ok(), "The processing execution unexpectedly panicked.");
    }
}
