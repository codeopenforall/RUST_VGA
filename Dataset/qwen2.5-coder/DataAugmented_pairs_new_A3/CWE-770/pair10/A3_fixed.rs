struct Manager;

impl Manager {
    const MAX_CAP: usize = 1000;

    fn allocate(&self, size: usize) -> Result<Vec<u8>, String> {
        if size > Self::MAX_CAP {
            return Err(format!(
                "Requested size {} exceeds the maximum allowed {}",
                size, Self::MAX_CAP
            ));
        }
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 1);
        Ok(vec)
    }

    fn process(&self, count: usize) -> Result<(), String> {
        let data = self.allocate(count)?;
        let mut handles = Vec::new();
        let mut total: u64 = 0;

        // Simulate processing logic
        for _ in 0..count {
            handles.push(total);
            total += 1;
        }

        Ok(())
    }
}

struct Processor;

impl Processor {
    fn process(&self, count: usize) -> Result<(), String> {
        let instance = Manager;
        instance.process(count)
    }
}