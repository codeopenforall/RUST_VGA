pub struct ResourceManager;

impl ResourceManager {
    pub fn execute(&self, count: usize) -> Result<u32, &'static str> {
        const MAX_COUNT: usize = 50;
        if count > MAX_COUNT {
            return Err("Input exceeds permitted limit");
        }
        let mut buffer = vec![0u32; MAX_COUNT];
        for i in 0..count {
            buffer[i] = 42;
        }
        Ok(buffer[count - 1])
    }
}