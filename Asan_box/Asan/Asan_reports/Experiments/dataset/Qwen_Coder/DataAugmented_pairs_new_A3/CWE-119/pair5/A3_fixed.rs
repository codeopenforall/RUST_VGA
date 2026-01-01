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

/*
This oracle test is designed to verify the integrity of the memory block.
It calls the run() function and asserts that the sentinel remains intact.
For the vulnerable version run() will return false (sentinel corrupted)
and this test will fail.
For the fixed version, run() returns true and the test passes.
*/
#[cfg(test)]
mod tests {
    use super::run;
    #[test]
    fn oracle_test() {
        let integrity = run();
        assert!(integrity, "Integrity check failed: sentinel was corrupted");
    }
}
