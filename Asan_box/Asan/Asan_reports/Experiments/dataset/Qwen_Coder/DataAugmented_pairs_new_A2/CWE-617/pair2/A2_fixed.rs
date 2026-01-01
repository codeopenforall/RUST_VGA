use std::sync::Arc;
use std::thread;

trait Operation {
    fn process(&self, index: usize) -> Result<(), &'static str>;
}

struct Container {
    elements: Vec<u32>,
}

impl Container {
    fn create() -> Self {
        Self {
            elements: vec![50, 150, 200, 75, 125],
        }
    }
}

impl Operation for Container {
    fn process(&self, index: usize) -> Result<(), &'static str> {
        if index >= self.elements.len() {
            return Err("Index out-of-range");
        }
        let value = self.elements[index];
        if value <= 100 {
            return Err("Invariant violation: element is not > 100");
        }
        Ok(())
    }
}

fn run_task(index: usize, cnt: Arc<Container>) -> Result<(), &'static str> {
    let cnt_safe = Arc::clone(&cnt);
    let handle = thread::spawn(move || {
        cnt_safe.process(index)
    });
    handle.join().map_err(|_| "Thread join failed")?
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut user_index: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    let container = Arc::new(Container::create());
    if user_index >= container.elements.len() || container.elements[user_index] <= 100 {
        eprintln!("Provided index is unsafe. Falling back to index = 2.");
        user_index = 2;
    }
    match run_task(user_index, container) {
        Ok(_) => println!("Processing completed successfully."),
        Err(err) => println!("Processing failed: {}", err),
    }
}

////////////////////// Test Oracle //////////////////////
// This test oracle function is designed so that it fails when executed
// against the vulnerable version (which panics due to a reachable assertion)
// and passes when executed against the corrected version (which safely handles invalid input).
// The test spawns a thread to run the processing operation using an attacker-controlled index (0)
// which, in the vulnerable code, triggers a panic.  In the corrected version, the operation results in
// an error return without panicking.
#[cfg(test)]
mod tests {
    use std::panic;
    use std::sync::Arc;
    use super::{run_task, Container};

    #[test]
    fn oracle_test() {
        let holder = Arc::new(Container::create());
        // Using index 0, which is known to trigger the invariant in the vulnerable version.
        let result = panic::catch_unwind(|| {
            // In the vulnerable code, this call panics due to the assert!
            // In the corrected version, it should return an error without panicking.
            let _ = run_task(0, Arc::clone(&holder));
        });
        // The oracle expects that the call does NOT panic.
        assert!(result.is_ok(), "The operation panicked unexpectedly");
    }
}
///////////////////////////////////////////////////////
