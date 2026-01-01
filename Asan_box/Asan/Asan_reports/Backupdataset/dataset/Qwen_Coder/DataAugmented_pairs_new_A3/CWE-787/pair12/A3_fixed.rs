struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Buffer { data: vec![0u8; size] }
    }

    unsafe fn write_data(&mut self, offset: usize, input: &[u8]) {
        if offset + input.len() > self.data.len() {
            panic!("Out-of-bound write prevented");
        }
        let dst = self.data.as_mut_ptr().add(offset);
        std::ptr::copy_nonoverlapping(input.as_ptr(), dst, input.len());
    }
}