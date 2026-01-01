use std::ptr;
use std::slice;

pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Buffer {
            data: vec![0; size],
        }
    }

    pub fn copy_from(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.data.len() {
            return Err("Input larger than buffer");
        }

        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), self.data.as_mut_ptr(), input.len());
        }

        Ok(())
    }

    pub fn first(&self) -> u8 {
        self.data[0]
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;

    // The test oracle checks that the first byte of the buffer is correctly set to 42.
    // In the vulnerable implementation, due to the offset error, the first byte remains 0,
    // causing the test to fail. In the fixed implementation, the test passes.
    #[test]
    fn memory_bounds_oracle() {
        let input: Vec<u8> = vec![42, 10, 20, 30, 40];
        let mut buf = Buffer::new(input.len());
        buf.copy_from(&input).expect("Copy failed");
        // Assert that the first element matches the expected first byte.
        assert_eq!(buf.first(), 42, "Expected first byte to be 42");
    }
}
