use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

struct Data {
    value: usize,
}

impl Data {
    unsafe fn update(&mut self, new_value: usize) {
        let ptr = &mut self.value as *mut usize;
        *ptr = new_value;
    }

    fn validate(&self) -> Result<(), &'static str> {
        if self.value < 100 {
            Ok(())
        } else {
            Err("Value too high")
        }
    }
}

pub fn execute(input: usize) -> Result<(), &'static str> {
    let shared = Arc::new(Mutex::new(Data { value: 0 }));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || -> Result<(), &'static str> {
        let mut data = shared_clone.lock().unwrap();
        unsafe {
            data.update(input);
        }
        data.validate()
    });

    match handle.join() {
        Ok(result) => result,
        Err(_) => Err("Thread join failed"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };

    match execute(input) {
        Ok(()) => println!("Execution completed successfully."),
        Err(e) => println!("Execution failed: {}", e),
    }
}

#[cfg(test)]
mod tests {
    // Import the execute function from the module under test.
    // When running the test on the vulnerable version, an input of 150 will trigger a panic.
    // For the fixed version, the error is returned gracefully without panicking.
    use super::execute;
    
    #[test]
    fn test_boundary() {
        let test_input = 150;
        let result = std::panic::catch_unwind(|| {
            // For the vulnerable code this will panic.
            // For the fixed code, this should return an error and not panic.
            execute(test_input)
        });
        assert!(
            result.is_ok(),
            "Execution panicked with an input equal or above the threshold"
        );
    }
}
