use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum MyError {
    Timeout,
    Cancelled,
}

struct TaskExecutor {
    stop: *mut bool,
}

impl TaskExecutor {
    fn new(signal: &mut bool) -> TaskExecutor {
        TaskExecutor { stop: signal as *mut bool }
    }

    fn execute(&self) -> Result<u32, MyError> {
        unsafe {
            if *self.stop {
                return Err(MyError::Cancelled); 
            }
        }
        thread::sleep(Duration::from_millis(10));
        Err(MyError::Timeout)
    }
}

fn main() {
    let mut flag = true;
    let executor = TaskExecutor::new(&mut flag);
    match executor.execute() {
        Ok(val) => println!("Operation completed successfully with: {}", val),
        Err(e) => println!("Operation failed with error: {:?}", e),
    }
}