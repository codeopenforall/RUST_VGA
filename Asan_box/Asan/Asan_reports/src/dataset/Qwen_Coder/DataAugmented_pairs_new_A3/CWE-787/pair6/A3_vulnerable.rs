use std::ptr;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        unsafe {
            v.set_len(size);
        }
        Buffer { data: v }
    }
    fn assign(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(index) = value;
        }
        Ok(())
    }
    fn content(&self) -> &[u32] {
        &self.data
    }
}
fn main() {
    let mut buf = Buffer::new(10);
    match buf.assign(10, 999) {
        Ok(()) => println!("Assignment completed (vulnerable)."),
        Err(e) => println!("Error: {}", e),
    }
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
