use std::vec::Vec;
struct DataBuffer {
    data: Vec<u8>,
}
impl DataBuffer {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        unsafe {
            v.set_len(n + 1); // Vulnerable: setting length to n + 1 instead of n
        }
        DataBuffer { data: v }
    }
    fn fill(&mut self, value: u8) {
        for i in 0..self.data.len() {
            self.data[i] = value;
        }
    }
}
fn main() {
    let mut buf = DataBuffer::new(5);
    buf.fill(10);
    println!("Buffer contents: {:?}", buf.data);
}