use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Application;

impl Application {
    fn execute(&self) -> Result<u32, &'static str> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let data = 42u32;
            let raw_ptr = &data as *const u32;
            thread::sleep(Duration::from_millis(150));
            unsafe {
                let _ = tx.send(*raw_ptr);
            }
        });
        let result = rx.recv_timeout(Duration::from_millis(100));
        match result {
            Ok(num) => Ok(num),
            Err(_) => Err("Operation timed out"),
        }
    }
}

fn main() {
    let app = Application;
    match app.execute() {
        Ok(value) => println!("Operation succeeded with value: {}", value),
        Err(err) => eprintln!("Operation failed: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::Application;

    // This test calls the execute method expecting an error because the operation should time out.
    #[test]
    fn test_timeout_handling() {
        let app = Application;
        match app.execute() {
            Ok(val) => panic!("Expected error due to timeout, but got successful result: {}", val),
            Err(err) => assert_eq!(err, "Operation timed out", "Expected a timeout error"),
        }
    }
}
