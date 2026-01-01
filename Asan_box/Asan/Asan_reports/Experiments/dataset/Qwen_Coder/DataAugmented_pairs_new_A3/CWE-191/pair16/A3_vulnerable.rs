use std::sync::{Arc, Mutex};
use std::thread;

struct DataHolder {
    count: u32,
}

impl DataHolder {
    unsafe fn decrement(&mut self, decrement_by: u32) -> u32 {
        let ptr: *mut u32 = &mut self.count;
        *ptr = *ptr - decrement_by;
        *ptr
    }
}

fn underflow(data: Arc<Mutex<DataHolder>>, decrement_amount: u32) -> Result<u32, &'static str> {
    let thread_handle = thread::spawn(move || {
        let mut holder = data.lock().unwrap();
        let new_value = holder.decrement(decrement_amount);
        Ok(new_value)
    });
    thread_handle.join().unwrap()
}

fn main() {
    let initial_data = DataHolder { count: 0 };
    let shared_data = Arc::new(Mutex::new(initial_data));
    let outcome = underflow(shared_data.clone(), 1);
    match outcome {
        Ok(value) => println!("Outcome: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

///////////////////////////////////////////////////////////////
// Test Oracle Function
// This test function invokes the process() routine with a triggering
// input that causes an integer underflow (attempting to subtract 1
// from an initial value of 0).
//
// The contract is as follows:
// - For the vulnerable version, process() will return Ok(4294967295)
//   due to wrap-around, causing the test to fail because an error was expected.
// - For the fixed version, process() will return Err("Integer underflow detected"),
//   and the test will pass.
///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_underflow_protection() {
        // Create a Container with initial value 0.
        let container = Container { value: 0 };
        let shared = Arc::new(Mutex::new(container));
        let result = process(shared.clone(), 1);
        // The test expects an error due to underflow protection.
        assert!(result.is_err(), "Underflow was not properly handled");
    }
}
