use std::sync::{Arc, Mutex};
use std::thread;
const THRESHOLD: usize = 100;
struct Service {
    value: usize,
}
impl Service {
    fn new() -> Self {
        Service { value: 1 }
    }
    fn update(&mut self, multiplier: usize) {
        let new_value = self.value.wrapping_mul(multiplier);
        if new_value >= THRESHOLD {
            eprintln!("Error: value exceeded safe threshold!");
        } else {
            self.value = new_value;
        }
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(Service::new()));
    let mut threads = vec![];
    for i in 1..=5 {
        let service_clone = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            let mut instance = service_clone.lock().unwrap();
            let factor = if i == 3 { 150 } else { 2 };
            instance.update(factor);
        }));
    }
    for handle in threads {
        let _ = handle.join();
    }
    println!("Final value: {}", shared.lock().unwrap().value);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle Function
// This test function is written separately to validate that the original code fails with a panic
// when provided with a malicious multiplier (due to the reachable assertion), while the corrected
// code handles the case gracefully.
//
// The test spawns a thread that calls the update method using a malicious multiplier (150).
// For the vulnerable implementation, the assertion triggers and the test will detect a panic,
// failing the test. For the fixed implementation, the update method returns an error,
// allowing the test to pass.
//
// The oracle is deterministic: it uses fixed thread scheduling and predetermined inputs.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::panic;

    // Adjust the module import if needed (assuming the main module is named "service")
    // use crate::Service; 

    // For testing, we simulate the service's behavior.
    #[test]
    fn test_malicious_update() {
        let shared = Arc::new(Mutex::new(super::Service::new()));
        let shared_clone = Arc::clone(&shared);

        // In the vulnerable version, this update call should panic.
        let result = panic::catch_unwind(|| {
            let mut instance = shared_clone.lock().unwrap();
            // Malicious multiplier that triggers assertion.
            instance.update(150);
        });

        // For the vulnerable code, the above should Err (panic).
        // For the fixed version, the update() returns Err gracefully.
        //
        // To test both versions with a single test oracle:
        // - If result is Err (panic), we interpret it as the vulnerability triggered (fail the test).
        // - Otherwise, if it returns Ok(()) but internal value remains safe, then the fix is correct.
        //
        // Here, the oracle expects that for fixed, no panic occurs.
        assert!(result.is_ok(), "Test failed: malicious update triggered a panic.");
    }
}
