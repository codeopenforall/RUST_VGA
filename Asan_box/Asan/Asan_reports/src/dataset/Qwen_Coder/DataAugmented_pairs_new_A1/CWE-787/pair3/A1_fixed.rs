use std::ptr;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(len: usize) -> Self {
        Self { data: vec![0; len] }
    }
    fn compute(&mut self) -> u32 {
        let len = self.data.len();
        unsafe {
            let ptr = self.data.as_mut_ptr();
            for i in 0..len {
                ptr.add(i).write(1); // Corrected index from i+1 to i
            }
        }
        self.data.iter().sum()
    }
}
fn main() {
    let mut buf = Buffer::new(6);
    let result = buf.compute();
    println!("{}", result);
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
