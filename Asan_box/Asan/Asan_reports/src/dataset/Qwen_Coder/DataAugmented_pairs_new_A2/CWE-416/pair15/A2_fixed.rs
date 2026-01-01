use std::thread;
use std::sync::Arc;

trait Operation {
    fn process(&self) -> i32;
}

struct Handler;

impl Operation for Handler {
    fn process(&self) -> i32 {
        let shared = Arc::new(42);
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            *shared_clone
        });
        handle.join().unwrap()
    }
}

pub fn compute() -> i32 {
    let proc = Handler;
    proc.process()
}

fn main() {
    let result = compute();
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::compute;

    #[test]
    fn test_oracle() {
        // The expected result is 42. In the vulnerable implementation, the use-after-free
        // could lead to an incorrect value or a crash, whereas the corrected implementation
        // returns 42 reliably.
        let result = compute();
        assert_eq!(result, 42, "Test failed: expected 42, got {}", result);
    }
}
