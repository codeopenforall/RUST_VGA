struct Processor {
    factor: u32,
}

impl Processor {
    fn calculate(&self, input: u32) -> u32 {
        unsafe {
            let prod: u64 = (input as u64).wrapping_mul(self.factor as u64);
            prod as u32
        }
    }

    fn check(&self, input: u32) -> Result<u32, &'static str> {
        let result = self.calculate(input);
        if result == 0 {
            Err("Overflow detected")
        } else {
            Ok(result)
        }
    }
}