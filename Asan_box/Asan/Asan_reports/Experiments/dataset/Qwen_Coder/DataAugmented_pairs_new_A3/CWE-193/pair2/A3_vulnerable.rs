struct DataBuffer {
    data: Vec<u8>,
}

impl DataBuffer {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        unsafe { data.set_len(size); }
        for i in 0..size {
            data[i] = i as u8;
        }
        DataBuffer { data }
    }

    pub fn process(&self) -> u8 {
        unsafe {
            *self.data.get_unchecked(self.data.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fencepost_error() {
        // Create a buffer of size 10.
        let buffer = DataBuffer::new(10);
        // The correct behavior is to return the last valid element (which is 9).
        let expected = 9;
        let result = buffer.process();
        // If the off-by-one error occurs, the returned result will be undefined (and likely not 9).
        assert_eq!(result, expected, "Test failed: off-by-one error detected in element access.");
    }
}
