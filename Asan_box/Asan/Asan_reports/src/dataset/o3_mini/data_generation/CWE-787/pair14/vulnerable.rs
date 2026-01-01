use std::ptr;
pub struct Handler {
    buf: Vec<u8>,
}
impl Handler {
    pub fn new(size: usize) -> Self {
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        Self { buf }
    }
    pub fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.buf.as_mut_ptr();
            ptr.add(index).write(value);
        }
        Ok(())
    }
    pub fn get(&self, index: usize) -> Option<u8> {
        self.buf.get(index).copied()
    }
}
fn main() {
    let mut handler = Handler::new(10);
    let _ = handler.update(10, 255);
    unsafe {
        let val = *handler.buf.as_ptr().add(10);
        println!("Main (vulnerable): value at index 10 is {}", val);
    }
}


#[cfg(test)]
mod tests {
    // Import the common interface from the module under test.
    use super::*;

    #[test]
    fn test_out_of_bounds_update() {
        let mut handler = Handler::new(10);
        // For the fixed implementation, the update should return an error when writing out-of-bounds.
        // For the vulnerable version, update returns Ok despite writing out-of-bounds.
        let result = handler.update(10, 255);
        assert!(
            result.is_err(),
            "The update method should fail on an out-of-bound index, but it did not."
        );
    }
}
