struct MemoryBlock {
    internal: Vec<u8>,
    cap: usize,
    sentinel: u8,
}

impl MemoryBlock {
    pub fn new(size: usize, sentinel: u8) -> Self {
        let mut internal = vec![0; size];
        internal.push(sentinel);
        MemoryBlock {
            internal,
            cap: size,
            sentinel,
        }
    }

    pub fn set(&mut self, index: usize, value: u8) {
        if index < self.cap {
            unsafe {
                let ptr = self.internal.as_mut_ptr();
                *ptr.add(index) = value;
            }
        }
    }

    pub fn check_sentinel(&self) -> bool {
        self.internal[self.cap] == self.sentinel
    }
}

pub fn run() -> bool {
    let mut block = MemoryBlock::new(10, 0xFF);
    block.set(5, 0x55);
    block.check_sentinel()
}