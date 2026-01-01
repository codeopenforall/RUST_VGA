use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug)]
pub struct Config {
    pub value: u32,
    pub valid: bool,
}
pub fn load_config() -> Config {
    Config {
        value: 42,
        valid: true,
    }
}
pub fn run_threads() -> Vec<u32> {
    let config = Arc::new(Mutex::new(load_config()));
    let mut results = Vec::new();
    let mut handles = Vec::new();
    for _ in 0..4 {
        let config_clone = Arc::clone(&config);
        let h = thread::spawn(move || {
            let cfg = config_clone.lock().unwrap();
            if cfg.valid {
                cfg.value + 1
            } else {
                cfg.value - 1
            }
        });
        handles.push(h);
    }
    for h in handles {
        results.push(h.join().unwrap());
    }
    results
}
fn main() {
    let results = run_threads();
    for r in results {
        println!("result: {}", r);
    }
}


//////////////// Oracle Test Code Begin
// This is the test oracle function. It must be compiled separately and linked against either the vulnerable or fixed code.
// It asserts that the configuration is properly initialized (i.e. cfg.valid is true) and that the thread computations yield the expected result (42 + 1 = 43).
// Note: This file must not be merged with the main binary builds.

#[cfg(test)]
mod tests {
    use super::*;

    // A helper function to mimic the production processing.
    fn aggregate_result() -> Vec<u32> {
        run_threads()
    }

    #[test]
    fn test_config_integrity() {
        // Directly test the configuration load.
        let cfg = load_config();
        assert!(cfg.valid, "Configuration 'valid' flag is not properly initialized!");
    }

    #[test]
    fn test_thread_results() {
        let results = aggregate_result();
        // The expected result for each thread is: if valid is true, then value + 1, i.e. 43.
        for res in results {
            assert_eq!(res, 43, "Thread computation produced an unexpected result");
        }
    }
}
//////////////// Oracle Test Code End
