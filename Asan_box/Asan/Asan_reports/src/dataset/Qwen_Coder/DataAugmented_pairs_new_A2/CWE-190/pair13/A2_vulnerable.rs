use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    quantity: u32,
    item_size: u32,
}

impl Processor {
    fn new(quantity: u32, item_size: u32) -> Self {
        Processor { quantity, item_size }
    }

    fn calculate_size(&self) -> usize {
        (self.quantity.wrapping_mul(self.item_size)) as usize
    }

    fn execute(&self) {
        let size = self.calculate_size();
        let layout = unsafe { Layout::from_size_align_unchecked(size, 8) };
        unsafe {
            let ptr = alloc_zeroed(layout);
            for i in 0..size {
                *ptr.add(i) = 1;
            }
            dealloc(ptr, layout);
        }
        println!("Execution complete. Calculated size: {}", size);
    }
}

fn main() {
    let processor = Arc::new(Processor::new(2_000_000_000, 3));
    let status = Arc::new(Mutex::new(false));
    let mut threads = Vec::new();
    for _ in 0..4 {
        let p = Arc::clone(&processor);
        let s = Arc::clone(&status);
        let t = thread::spawn(move || {
            p.execute();
            let mut completed = s.lock().unwrap();
            *completed = true;
        });
        threads.push(t);
    }
    for t in threads {
        t.join().unwrap();
    }
    assert!(*status.lock().unwrap());
    println!("Main completed execution.");
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
