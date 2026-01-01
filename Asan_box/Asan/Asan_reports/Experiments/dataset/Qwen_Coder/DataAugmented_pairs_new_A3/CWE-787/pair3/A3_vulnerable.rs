pub struct Buffer {
    data: Vec<i32>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        let mut data = vec![0; size];
        let ptr = data.as_mut_ptr();
        for i in 0..size {
            // Intentionally writing to the next element, causing an off-by-one error
            unsafe { ptr.add(i + 1).write(1); }
        }
        Buffer { data }
    }

    pub fn compute(&self) -> i32 {
        self.data.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let mut buf = Buffer::new(6);
        let sum = buf.compute();
        // In the proper implementation, every element in the 6-length buffer is set to 1,
        // so the expected sum is 6.
        assert_eq!(sum, 6, "Buffer sum value is not as expected");
    }
}
