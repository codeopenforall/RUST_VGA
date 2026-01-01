struct DataContainer {
    buffer: Vec<i32>,
}

impl DataContainer {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        unsafe { v.set_len(size + 1); } // Intentional off-by-one error
        for i in 0..(size + 1) {
            v[i] = i as i32;
        }
        DataContainer { buffer: v }
    }

    pub fn sum(&self) -> i32 {
        self.buffer.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buffer_integrity() {
        // The intended behavior for a buffer of size 10 is to sum values 0..9, which equals: 0+1+2+...+9 = 45.
        let instance = DataContainer::new(10);
        let expected_sum = 45;
        assert_eq!(instance.sum(), expected_sum, "Buffer sum mismatch: off-by-one vulnerability present");
    }
}
