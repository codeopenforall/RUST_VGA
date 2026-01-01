struct DataHolder {
    buffer: Vec<u32>,
}

impl DataHolder {
    pub fn new() -> Self {
        DataHolder {
            buffer: Vec::new(),
        }
    }

    pub fn process(&mut self, input: &[u32]) {
        if input.len() > self.buffer.capacity() {
            self.buffer = Vec::with_capacity(input.len());
        }
        self.buffer.extend_from_slice(input);
    }

    pub fn total(&self) -> u32 {
        self.buffer.iter().sum()
    }
}

//////////////////////////////
// Test Oracle
//////////////////////////////
#[cfg(test)]
mod tests {
    // Import all items from the current crate.
    use super::*;

    #[test]
    fn test_total_sum() {
        // The triggering input: 11 elements.
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        // Expected sum is 66.
        let expected_sum: u32 = 66;
        let mut holder = DataHolder::new();
        holder.process(&input);
        assert_eq!(
            holder.total(),
            expected_sum,
            "The computed sum did not match the expected value."
        );
    }
}
