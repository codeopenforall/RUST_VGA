use std::sync::Arc;
use std::thread;
struct DataContainer {
    items: Vec<u32>,
}
impl DataContainer {
    fn new(count: usize) -> Self {
        DataContainer {
            items: (0..count as u32).collect(),
        }
    }
    fn read_item(&self, index: usize) -> Option<u32> {
        self.items.get(index).copied()
    }
}
fn main() {
    let container = Arc::new(DataContainer::new(10));
    let shared = container.clone();
    let handle = thread::spawn(move || {
        let result = shared.read_item(10);
        match result {
            Some(val) => println!("Thread read value: {}", val),
            None => println!("Out-of-bound access prevented"),
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
