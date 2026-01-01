struct DataBuffer {
    data: Vec<i32>,
}

impl DataBuffer {
    pub fn new(data: Vec<i32>) -> Self {
        DataBuffer { data }
    }

    pub fn get(&self, index: usize) -> Result<i32, &'static str> {
        if index < self.data.len() {
            unsafe {
                Ok(*self.data.as_ptr().add(index))
            }
        } else {
            Err("Index out of bounds")
        }
    }
}

pub fn execute(index: usize) -> Result<i32, &'static str> {
    let buffer = DataBuffer::new(vec![1, 2, 3]);
    buffer.get(index)
}

#[cfg(test)]
mod tests {
    use super::execute;

    // This test verifies the contract: when an out-of-bound index is used,
    // the correct error is returned in the fixed version.
    // In the vulnerable version, the function does not return an error,
    // so this test will fail.
    #[test]
    fn contract_test() {
        match execute(4) {
            Ok(val) => panic!("Contract violation: expected error for out-of-bound access, got value {}", val),
            Err(e) => assert_eq!(e, "Index out of bounds", "Unexpected error message"),
        }
    }
}
