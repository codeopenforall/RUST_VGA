use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct System {
    threshold: usize,
}

impl System {
    fn new(value: usize) -> Self {
        System { threshold: value }
    }

    fn process(&self) {
        debug_assert!(self.threshold < 100, "Threshold too high, possible DoS");
    }
}

fn run(args: Vec<String>) {
    let input = args.get(1).expect("Missing argument");
    let value: usize = input.parse().expect("Invalid number provided");
    if value >= 100 {
        panic!("Invalid threshold value");
    }
    let config = Arc::new(Mutex::new(System::new(value)));
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let sys = config_clone.lock().unwrap();
        sys.process();
    });
    {
        let sys = config.lock().unwrap();
        sys.process();
    }
    handle.join().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args);
}

#[cfg(test)]
mod tests {
    use std::panic;

    // The following tests simulate the application using predetermined arguments.

    // For the vulnerable version, providing a value that violates the invariant (>= 100)
    // should trigger a panic (failure).
    #[test]
    fn oracle_vulnerable_test() {
        // Simulate command-line arguments: second argument "150" causes assertion failure.
        let args = vec!["program".to_string(), "150".to_string()];
        let result = panic::catch_unwind(|| {
            // This function represents the entry point for the vulnerable version.
            crate::run(args);
        });
        assert!(result.is_err(), "Expected panic for invariant violation in vulnerable version");
    }

    // For the corrected version, providing a safe value should allow the application to run without panic.
    #[test]
    fn oracle_fixed_test() {
        // Simulate command-line arguments: second argument "50" is within the safe threshold.
        let args = vec!["program".to_string(), "50".to_string()];
        let result = panic::catch_unwind(|| {
            // This function represents the entry point for the fixed version.
            crate::run(args);
        });
        assert!(result.is_ok(), "Application panicked unexpectedly in the corrected version");
    }
}
