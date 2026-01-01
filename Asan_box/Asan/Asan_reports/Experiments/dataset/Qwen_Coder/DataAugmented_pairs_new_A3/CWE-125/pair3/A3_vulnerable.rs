pub struct DataHolder {
    data: Vec<i32>,
}

impl DataHolder {
    pub fn new(data: Vec<i32>) -> Self {
        DataHolder { data }
    }

    pub fn retrieve(&self, idx: usize) -> Option<i32> {
        unsafe { Some(*self.data.as_ptr().add(idx)) }
    }
}

#[cfg(test)]
mod tests {
    use super::DataHolder;

    #[test]
    fn test_out_of_bound_read() {
        // Create a DataHolder with a vector of three elements.
        // Attempt to retrieve an element at index 4 (which is out-of-bound).
        // In the safe (fixed) version, this should return None.
        // In the vulnerable version, the unchecked memory access will likely return Some(garbage)
        // or trigger undefined behavior. Thus, this test serves as a contract.
        let holder = DataHolder::new(vec![10, 20, 30]);
        let result = holder.retrieve(4);
        assert!(result.is_none(), "Expected None for out-of-bound read, but got {:?}", result);
    }
}
