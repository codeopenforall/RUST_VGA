use std::sync::Arc;
use std::thread;

struct DataHolder {
    elements: Vec<u32>,
}

impl DataHolder {
    fn new(size: usize) -> Self {
        DataHolder {
            elements: (0..size as u32).collect(),
        }
    }

    fn fetch_element(&self, idx: usize) -> Option<u32> {
        unsafe { Some(*self.elements.get_unchecked(idx)) }
    }
}

fn main() {
    let holder = Arc::new(DataHolder::new(10));
    let shared = holder.clone();
    let handle = thread::spawn(move || {
        let outcome = shared.fetch_element(10);
        match outcome {
            Some(val) => println!("Thread fetched value: {}", val),
            None => println!("Thread encountered None"),
        }
    });
    let _ = handle.join();
}

#[cfg(test)]
mod tests {
    use super::DataContainer;

    // Test oracle function:
    // For the fixed implementation, out-of-bound access should safely return None.
    // In the vulnerable implementation, the undefined behavior may return a value (or even cause a crash),
    // so this test is designed to fail against the vulnerable code.
    #[test]
    fn test_out_of_bounds_access() {
        let container = DataContainer::new(10);
        let result = container.read_item(10);
        // We expect None because the index 10 is out-of-bound (valid indices are 0..9)
        assert!(result.is_none(), "Expected None for out-of-bound access, but got a value.");
    }
}
