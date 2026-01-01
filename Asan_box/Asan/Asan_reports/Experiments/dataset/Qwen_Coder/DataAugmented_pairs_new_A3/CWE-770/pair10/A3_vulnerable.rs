struct Manager;

impl Manager {
    unsafe fn unsafe_allocate(&self, size: usize) -> Vec<u8> {
        let mut vec = Vec::with_capacity(size);
        vec.set_len(size);
        for elem in vec.iter_mut() {
            *elem = 1;
        }
        vec
    }

    fn process(&self, count: usize) -> Result<(), String> {
        let data = unsafe { self.unsafe_allocate(count) };
        let mut handles = Vec::new();
        let mut total: u64 = 0;

        for _ in 0..count {
            handles.push(data.clone());
            total += data.len() as u64;
        }

        Ok(())
    }
}

struct Processor;

impl Processor {
    fn process(&self, count: usize) -> Result<(), String> {
        let manager = Manager;
        manager.process(count)
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
