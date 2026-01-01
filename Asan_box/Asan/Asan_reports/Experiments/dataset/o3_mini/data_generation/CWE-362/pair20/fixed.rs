use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
struct SharedState {
    flag: AtomicBool,
}
impl SharedState {
    fn new() -> Self {
        SharedState { flag: AtomicBool::new(false) }
    }
    fn check_then_set(&self) -> bool {
        if self.flag.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            thread::sleep(Duration::from_millis(1));
            true
        } else {
            false
        }
    }
}
fn run_test() -> usize {
    let shared = Arc::new(SharedState::new());
    let mut handles = Vec::new();
    for _ in 0..10 {
        let s = shared.clone();
        handles.push(thread::spawn(move || s.check_then_set()));
    }
    let results: Vec<bool> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let success_count = results.into_iter().filter(|&x| x).count();
    println!("Number of successful accesses: {}", success_count);
    success_count
}
fn main() {
    run_test();
}


//////////////////////////////////////////////////////////////
// Test Oracle Function for Race Condition (CWE-362) Check  //
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // The oracle function is designed to be used with either the vulnerable or
    // the corrected code. It calls the run_test() function, expecting exactly 1
    // successful “check_then_set” invocation. For the vulnerable version, due to
    // the race condition, the test is expected to fail (i.e. returning a number
    // greater than 1). For the corrected version, it should pass.
    use super::run_test;

    #[test]
    fn test_oracle() {
        let successes = run_test();
        assert_eq!(successes, 1, "Race condition detected: expected exactly one successful access, but got {}.", successes);
    }
}
