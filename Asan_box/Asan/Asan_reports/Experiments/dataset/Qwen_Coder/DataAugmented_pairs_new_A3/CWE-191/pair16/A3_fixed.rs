use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    value: u32,
}

impl Container {
    fn subtract_unchecked(&mut self, amount: u32) -> Result<u32, &'static str> {
        if amount > self.value {
            Err("Integer underflow detected")
        } else {
            self.value -= amount;
            Ok(self.value)
        }
    }
}

fn process(data: Arc<Mutex<Container>>, amount: u32) -> Result<u32, &'static str> {
    let handle = thread::spawn(move || {
        let mut container = data.lock().unwrap();
        container.subtract_unchecked(amount)
    });
    handle.join().unwrap()
}

fn main() {
    let container = Container { value: 0 };
    let shared = Arc::new(Mutex::new(container));
    let result = process(shared.clone(), 1);
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("Error: {}", err),
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
