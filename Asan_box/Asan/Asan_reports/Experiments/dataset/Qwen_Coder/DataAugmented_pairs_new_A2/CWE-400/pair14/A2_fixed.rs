use std::thread;

pub trait Handler {
    fn process(&self, data: &[u64]) -> Result<u64, &'static str>;
}

pub struct ResourceManager;

impl Handler for ResourceManager {
    fn process(&self, data: &[u64]) -> Result<u64, &'static str> {
        const MAX_ITEMS: usize = 1000;
        if data.len() > MAX_ITEMS {
            return Err("Input size exceeds allowable limit");
        }
        let mut result = 0u64;
        if data.len() > 10 {
            let mut threads = Vec::with_capacity(data.len());
            for &item in data {
                let handle = thread::spawn(move || {
                    let mut val = item;
                    val *= 2;
                    val
                });
                threads.push(handle);
            }
            for th in threads {
                result = result.saturating_add(th.join().map_err(|_| "Thread error")?);
            }
        } else {
            for &item in data {
                let doubled = {
                    let mut val = item;
                    val *= 2;
                    val
                };
                result = result.saturating_add(doubled);
            }
        }
        Ok(result)
    }
}

fn main() {
    let manager = ResourceManager;
    let data = vec![1, 2, 3, 4];
    match manager.process(&data) {
        Ok(total) => println!("Result: {}", total),
        Err(e) => println!("Error: {}", e),
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
