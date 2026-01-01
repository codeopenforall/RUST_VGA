use std::ptr;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        v.resize(size, 0);
        Buffer { data: v }
    }
    fn assign(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index >= self.data.len() {
            return Err("Index out of bounds");
        }
        self.data[index] = value;
        Ok(())
    }
    fn content(&self) -> &[u32] {
        &self.data
    }
}
fn main() {
    let mut buf = Buffer::new(10);
    buf.assign(9, 999).expect("Assignment failed");
    println!("Assignment completed (fixed).");
    println!("Buffer content: {:?}", buf.content());
}

#[cfg(test)]
mod tests {
    // Import from the module where Buffer is defined.
    use super::*;

    #[test]
    fn test_out_of_bounds() {
        // Create a buffer of size 10.
        let mut buffer = Buffer::new(10);
        // Attempt to write to an out-of-bound index.
        // For the corrected code, this should return an error.
        let result = buffer.assign(10, 42);
        assert!(result.is_err(), "Buffer assignment with out-of-bounds index should fail.");
    }
}
