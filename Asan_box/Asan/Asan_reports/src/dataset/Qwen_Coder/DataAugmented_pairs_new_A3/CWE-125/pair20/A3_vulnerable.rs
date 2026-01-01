pub struct DataContainer {
    items: Vec<i32>,
}

impl DataContainer {
    pub fn new(size: usize) -> Self {
        DataContainer {
            items: vec![0; size],
        }
    }

    pub fn read_item(&self, index: usize) -> Option<i32> {
        unsafe {
            Some(*self.items.get_unchecked(index))
        }
    }
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
