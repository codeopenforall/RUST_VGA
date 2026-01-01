use std::thread;

trait Operation {
    fn process(&self) -> i32;
}

struct Handler;

impl Operation for Handler {
    fn process(&self) -> i32 {
        let boxed = Box::new(42);
        let raw_ptr = Box::into_raw(boxed);
        unsafe {
            let handle = thread::spawn(|| {
                let _dummy = vec![0u8; 1024];
            });
            handle.join().unwrap();
            let result = *raw_ptr; // Access the value before dropping
            drop(Box::from_raw(raw_ptr)); // Drop the box
            result
        }
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
