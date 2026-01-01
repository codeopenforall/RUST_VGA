use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

struct Processor {
    state: Arc<AtomicUsize>,
    buffer: Vec<u8>,
}

impl Processor {
    fn new() -> Self {
        Self {
            state: Arc::new(AtomicUsize::new(64)),
            buffer: vec![0; 64],
        }
    }

    fn execute(&self, index: usize) {
        let state_handle = Arc::clone(&self.state);
        let state_at_spawn = self.state.load(Ordering::Relaxed);

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            state_handle.store(0, Ordering::Relaxed);
        });

        thread::sleep(Duration::from_millis(20));

        unsafe {
            let ptr = self.buffer.as_ptr().add(index);
            let current_bound = state_at_spawn; // Use the state at the time of spawn
            assert!(index < current_bound, "Invariant violation: index out of bound");
            println!("Buffer value: {}", *ptr);
        }
    }
}

fn main() {
    let proc = Processor::new();
    proc.execute(32);
}

///////////////////////////////
// Test Oracle Function
///////////////////////////////

// This test oracle function is intended to be compiled separately (e.g., as part of a test suite).
// It invokes the processing operation with a controlled index value (32).
// In the vulnerable version, the background thread causes the invariant to fail,
// leading the function to panic. In the fixed version, the invariant is preserved,
// allowing the function to complete normally.

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        // The test asserts that the processing operation completes without panicking.
        // In the vulnerable version, this test will fail (catching a panic),
        // whereas in the fixed version, it will pass.
        let result = std::panic::catch_unwind(|| {
            let proc = Processor::new();
            proc.execute(32);
        });
        assert!(result.is_ok(), "Processing operation panicked due to invariant violation");
    }
}
