use std::vec::Vec;

struct DataBuffer {
    data: Vec<u8>,
}

impl DataBuffer {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        v.resize(n, 0); // Initialize the vector with zeros
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