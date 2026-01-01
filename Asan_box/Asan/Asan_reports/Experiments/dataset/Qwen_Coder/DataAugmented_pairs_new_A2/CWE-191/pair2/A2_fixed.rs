use std::sync::{Arc, Mutex};
use std::thread;

struct Record {
    value: u32,
}

impl Record {
    fn new(init: u32) -> Self {
        Record { value: init }
    }

    fn decrease(&mut self, deduction: u32) -> Result<(), &'static str> {
        if deduction > self.value {
            Err("Integer underflow prevented: deduction exceeds current value")
        } else {
            self.value -= deduction;
            Ok(())
        }
    }
}

pub fn run_process() -> u32 {
    let data = Arc::new(Mutex::new(Record::new(50)));
    let mut threads = Vec::new();

    for _ in 0..5 {
        let data_ref = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let deduction = 20u32;
            let mut guard = data_ref.lock().unwrap();
            let _ = guard.decrease(deduction);
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    let final_value = data.lock().unwrap().value;
    println!("Final value: {}", final_value);
    final_value
}

fn main() {
    let _ = run_process();
}

#[cfg(test)]
mod tests {
    // The test oracle validates that the final outcome is as expected.
    // For the corrected version, after 5 concurrent attempts to subtract 20 from an initial value of 50,
    // only the first two deductions succeed (50 -> 30 -> 10). Subsequent attempts do not change the state due to underflow prevention.
    // Thus, the final value must be exactly 10.
    use super::run_process;

    #[test]
    fn test_underflow() {
        // Execute the process and capture the final output.
        let result = run_process();
        // Assert that the final value is 10.
        assert_eq!(result, 10, "Test failed: Detected integer underflow vulnerability.");
    }
}
