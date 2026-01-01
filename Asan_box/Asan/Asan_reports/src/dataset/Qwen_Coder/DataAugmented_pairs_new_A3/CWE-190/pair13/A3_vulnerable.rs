use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::sync::{Arc, Mutex};
use std::thread;

struct Handler {
    count: u32,
    element_size: u32,
}

impl Handler {
    fn new(count: u32, element_size: u32) -> Self {
        Handler { count, element_size }
    }

    fn compute_size(&self) -> usize {
        (self.count.wrapping_mul(self.element_size)) as usize
    }

    fn process(&self) {
        let size = self.compute_size();
        let layout = Layout::from_size_align(size, 8).expect("Invalid layout");
        unsafe {
            let ptr = alloc_zeroed(layout);
            for i in 0..size {
                *ptr.add(i) = 1;
            }
            dealloc(ptr, layout);
        }
        println!("Processing complete. Computed size: {}", size);
    }
}

fn main() {
    let handler = Arc::new(Handler::new(2_000_000_000, 3));
    let flag = Arc::new(Mutex::new(false));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let h = Arc::clone(&handler);
        let f = Arc::clone(&flag);
        let t = thread::spawn(move || {
            h.process();
            let mut done = f.lock().unwrap();
            *done = true;
        });
        handles.push(t);
    }
    for t in handles {
        t.join().expect("Thread failed");
    }
    assert!(*flag.lock().unwrap());
    println!("Main finished execution.");
}

#[cfg(test)]
mod tests {
    use super::*;
    // This oracle function creates a Handler with parameters that, when multiplied correctly,
    // should yield 6_000_000_000. In the vulnerable code, the multiplication wraps, returning an incorrect value.
    // The test asserts that the computed size equals the mathematically correct value.
    fn safe_product(count: u32, element_size: u32) -> usize {
        // Compute using u64 arithmetic to avoid overflow.
        let prod = (count as u64) * (element_size as u64);
        prod as usize
    }

    #[test]
    fn test_allocation_size() {
        let count = 2_000_000_000;
        let element_size = 3;
        let handler = Handler::new(count, element_size);
        let computed = handler.compute_size();
        let expected = safe_product(count, element_size);
        // For the vulnerable code, computed != expected; for the fixed code, they are equal.
        assert_eq!(computed, expected, "Computed size does not match expected safe multiplication result");
    }
}
