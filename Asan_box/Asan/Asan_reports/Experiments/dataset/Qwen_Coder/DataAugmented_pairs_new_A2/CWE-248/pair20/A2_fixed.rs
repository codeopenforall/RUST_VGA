use std::thread;
trait Execute {
    fn execute(&self) -> Result<(), ()>;
}
struct Runner;
impl Execute for Runner {
    fn execute(&self) -> Result<(), ()> {
        let handle = thread::spawn(|| {
            let num_ptr = &10 as *const i32;
            let value = unsafe { *num_ptr };
            if value == 10 {
                panic!("Thread panic captured safely");
            }
        });
        match handle.join() {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Recovered from thread panic: {:?}", err);
                Ok(())
            }
        }
    }
}
pub fn run_logic() -> Result<(), ()> {
    let runner = Runner;
    runner.execute()
}
fn main() {
    if run_logic().is_ok() {
        println!("Execution completed safely");
    } else {
        println!("Execution encountered errors");
    }
}

/////////////////////////////////////////////////////////////
// Test Oracle: This function serves as a unit test to validate
// that the core logic in the library does not cause an unguarded panic.
// For the vulnerable variant, this test will fail as the panic propagates.
// In the corrected variant, the panic is caught, and the test passes.
/////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;
    
    // Import the run_logic function from the module under test.
    use crate::run_logic;

    #[test]
    fn test_run_logic_behavior() {
        // Execute the core logic in a catch_unwind closure.
        // In the vulnerable variant, this will catch an Err(..) result.
        // In the corrected variant, run_logic completes and returns Ok(()).
        let result = catch_unwind(|| {
            run_logic();
        });
        
        // We require that run_logic executes without causing an unhandled panic.
        // For the vulnerable code, result.is_err() causing the test to fail.
        // For the fixed code, result.is_ok() and the test passes.
        assert!(result.is_ok(), "The function panicked unexpectedly");
    }
}
