pub struct Buffer {
    data: Vec<i32>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        let mut buffer = Buffer {
            data: vec![0; size],
        };
        for i in 0..size {
            unsafe {
                let ptr = buffer.data.as_mut_ptr();
                ptr.add(i).write(1);
            }
        }
        buffer
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
