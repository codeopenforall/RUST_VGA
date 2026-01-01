use std::ptr;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        Buffer { data: vec![0; size] }
    }
    fn fill_data(&mut self, value: u32) {
        unsafe {
            let n = self.data.len();
            let ptr = self.data.as_mut_ptr();
            for i in 0..n {
                ptr.add(i).write(value);
            }
        }
    }
}
fn main() {
    let mut buf = Buffer::new(10);
    buf.fill_data(55);
    println!("Buffer content: {:?}", buf.data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_fill() {
        let mut buf = Buffer::new(10);
        buf.fill_data(42);
        // Expect the vector length to be exactly 10.
        // In the vulnerable version, the length becomes 11 due to the extra write,
        // causing this test to fail.
        assert_eq!(buf.data.len(), 10, "Buffer length is incorrect; vulnerability detected.");

        for &val in &buf.data {
            assert_eq!(val, 42, "Buffer element was not filled correctly.");
        }
    }
}
