use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum MyError {
    Timeout,
    Cancelled,
}

struct TaskRunner {
    cancel: *mut bool,
}

impl TaskRunner {
    fn new(flag: &mut bool) -> TaskRunner {
        TaskRunner { cancel: flag as *mut bool }
    }

    fn run(&self) -> Result<u32, MyError> {
        unsafe {
            if *self.cancel {
                return Err(MyError::Cancelled);
            }
        }
        thread::sleep(Duration::from_millis(10));
        Err(MyError::Timeout)
    }
}

fn main() {
    let mut flag = true;
    let runner = TaskRunner::new(&mut flag);
    match runner.run() {
        Ok(val) => println!("Operation completed successfully with: {}", val),
        Err(e) => println!("Operation failed with error: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_test() {
        // Set the cancellation flag to trigger the exceptional condition.
        let mut flag = true;
        let runner = TaskRunner::new(&mut flag);
        let result = runner.run();
        // The correct behavior (in the fixed version) is to return a cancellation error.
        // The vulnerable version erroneously returns Ok, so this test will fail for it.
        assert_eq!(result, Err(MyError::Cancelled), "Expected cancellation error with flag set to true");
    }
}
