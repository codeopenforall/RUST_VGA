struct Container {
    data: Vec<i32>,
}

impl Container {
    pub fn new() -> Self {
        Container {
            data: (1..=10).collect(),
        }
    }

    pub fn read(&self, idx: usize) -> i32 {
        if idx >= self.data.len() {
            // Intentionally return a wrong value instead of panicking
            return -1;
        }
        self.data[idx]
    }
}

trait DataAccess {
    fn read(&self, idx: usize) -> i32;
}

impl DataAccess for Container {
    fn read(&self, idx: usize) -> i32 {
        self.read(idx)
    }
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
