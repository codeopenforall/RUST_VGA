use std::sync::Arc;
use std::thread;

struct Data {
    value: i32,
}

impl Data {
    unsafe fn compute(&self) -> i32 {
        if self.value < 0 {
            panic!("abnormal termination");
        }
        self.value * 2
    }
}

fn launch(data: Arc<Data>) -> thread::JoinHandle<i32> {
    thread::spawn(move || {
        unsafe { data.compute() }
    })
}

pub fn execute(data: Arc<Data>) -> Result<i32, &'static str> {
    let handle = launch(data);
    // Introduce a defect: unwrap instead of handling the result
    Ok(handle.join().unwrap())
}

fn main() {
    let data = Arc::new(Data { value: -1 });
    let _result = execute(data);
    println!("Completed execution");
}

//////////////////////////////////////////////////////////////
// Test Oracle Function for Exception Handling Behavior
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::panic;
    // Import the execute function and the Data struct from the target code.
    // The same oracle code is used to test both versions.
    // When running against the vulnerable version, the unsafely unwrapped panic 
    // will cause the test to fail; when run against the fixed version, the error
    // is gracefully handled.
    use super::{execute, Data};

    #[test]
    fn test_exception_handling() {
        // Create an input designed to trigger the error condition.
        let data = Arc::new(Data { value: -1 });
        // Execute within a catch_unwind to intercept any panics.
        let result = panic::catch_unwind(|| execute(data.clone()));
        match result {
            // If no panic occurred, we expect the function to return an error Result.
            Ok(inner) => {
                // The execute function returns a Result; for a negative input,
                // it should be an Err variant indicating that the exception was handled.
                assert!(
                    inner.is_err(),
                    "Expected error result, but got an Ok value with {:?}",
                    inner.ok()
                );
            },
            // If a panic was caught, it means the exception crossed the boundary.
            // This is indicative of the vulnerability.
            Err(_) => {
                panic!("Uncaught exception detected: the error was not handled gracefully");
            }
        }
    }
}
