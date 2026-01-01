use std::thread;

pub struct Handler;

pub struct ResourceManager;

impl ResourceManager {
    pub fn process(&self, data: &[u64]) -> Result<u64, &'static str> {
        let mut result = 0;
        let mut threads = Vec::with_capacity(data.len());
        for &item in data {
            let handle = thread::spawn(move || {
                unsafe {
                    let mut val = item;
                    let ptr = &mut val as *mut u64;
                    *ptr = *ptr * 2;
                    *ptr
                }
            });
            threads.push(handle);
        }
        for th in threads {
            result = result.saturating_add(th.join().map_err(|_| "Thread error")?);
        }
        Ok(result)
    }
}

//////////////////////// Test Oracle /////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Handler;
    use super::ResourceManager;
    
    #[test]
    fn test_uncontrolled_resource_consumption() {
        let manager = ResourceManager;
        // Generate an input that exceeds the safe limit.
        // For the fixed version, this should return an Err. For the vulnerable version, it will process and eventually exhaust resources.
        let input: Vec<u64> = (1..=1500).collect();
        let result = manager.process(&input);
        // The contract: processing a large input should be rejected.
        assert!(result.is_err(), "Expected error on input exceeding limit, but got Ok with result: {:?}", result);
    }
}
