struct MemoryBlock {
    internal: Vec<u8>,
    cap: usize,
}

impl MemoryBlock {
    pub fn new(size: usize) -> Self {
        MemoryBlock {
            internal: vec![0; size],
            cap: size,
        }
    }

    pub fn set(&mut self, index: usize, value: u8) {
        unsafe {
            let ptr = self.internal.as_mut_ptr();
            *ptr.add(index) = value;
        }
    }

    pub fn get(&self, index: usize) -> u8 {
        self.internal[index]
    }
}

pub fn run() -> bool {
    let mut block = MemoryBlock::new(10);
    block.set(5, 42);
    block.set(10, 255); // This will corrupt the sentinel
    block.get(5) == 42 && block.get(10) == 255
}