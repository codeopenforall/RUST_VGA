use std::sync::Arc;
use std::thread;

struct Container {
    elements: Vec<i32>,
}

impl Container {
    fn create() -> Self {
        Container {
            elements: vec![50, 150, 250],
        }
    }

    unsafe fn process(&self, index: usize) {
        let ptr = self.elements.as_ptr();
        let value = *ptr.add(index);
        assert!(value > 100, "Invariant failure: element is not > 100");
    }
}

fn run_task(index: usize, cnt: Arc<Container>) {
    let handle = thread::spawn(move || unsafe {
        cnt.process(index);
    });
    handle.join().unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user_index: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    let shared = Arc::new(Container::create());
    run_task(user_index, shared);
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
