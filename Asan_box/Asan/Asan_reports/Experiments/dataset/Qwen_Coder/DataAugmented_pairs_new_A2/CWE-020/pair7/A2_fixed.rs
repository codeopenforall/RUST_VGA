use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

trait DataAccess {
    fn read(&self, idx: usize) -> i32;
}

struct Container {
    data: Vec<i32>,
}

impl Container {
    fn new() -> Self {
        Container { data: (1..=10).collect() }
    }
}

impl DataAccess for Container {
    fn read(&self, idx: usize) -> i32 {
        if idx >= self.data.len() {
            panic!("Index out of bounds");
        }
        self.data[idx]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let idx: usize = args
        .get(1)
        .unwrap_or(&"0".to_string())
        .parse()
        .expect("Invalid number provided");
    let container = Arc::new(Container::new());
    let result = Arc::new(Mutex::new(0));
    let cont_clone = Arc::clone(&container);
    let res_clone = Arc::clone(&result);
    let handle = thread::spawn(move || {
        let val = cont_clone.read(idx);
        let mut res = res_clone.lock().unwrap();
        *res = val;
    });
    handle.join().expect("Thread panicked");
    let final_value = *result.lock().unwrap();
    println!("Value: {}", final_value);
}

#[cfg(test)]
mod tests {
    use std::panic;
    // Import the Container struct and its associated trait implementation.
    // The code under test (either vulnerable or corrected) must define these.
    use super::{Container, DataAccess};

    #[test]
    fn test_valid_index() {
        let container = Container::new();
        // For index 2, expect the value 3 (since Container::new fills 1..=10).
        assert_eq!(container.read(2), 3);
    }

    #[test]
    fn test_out_of_bounds() {
        let container = Container::new();
        // For the fixed code, an out-of-bounds access should trigger a panic.
        let result = panic::catch_unwind(|| {
            // Deliberately use an index that is out of bounds.
            container.read(15);
        });
        // In the fixed version, this should be an error; in the vulnerable version,
        // the undefined behavior might not panic, causing the test to fail.
        assert!(result.is_err(), "Out-of-bound read did not panic as expected");
    }
}
