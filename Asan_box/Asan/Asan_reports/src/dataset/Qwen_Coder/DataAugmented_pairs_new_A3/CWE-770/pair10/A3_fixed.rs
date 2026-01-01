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

//////////////////////////////
// Test Oracle Function     //
// This test function should be compiled and run separately.
// It verifies that supplying an unreasonably high count results in an error in the corrected code,
// while the vulnerable version does not enforce the limit and would erroneously succeed.
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::Manager;
    use super::Processor;

    #[test]
    fn test_resource_cap() {
        let instance = Manager;
        // Use an input count that exceeds the allowed maximum.
        let excessive_count = 1500;
        let result = instance.process(excessive_count);
        // In the corrected version, an error is expected due to the cap,
        // while in the vulnerable version, the resource limit is not enforced.
        assert!(result.is_err(), "Expected an error when exceeding the resource cap.");
    }
}
