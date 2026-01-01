use std::thread;

struct ResourceManager;

impl ResourceManager {
    pub fn process_request(count: usize, size: usize) -> Result<u8, &'static str> {
        if count > 1000 || size > 10 * 1024 * 1024 {
            return Err("Resource limit exceeded");
        }

        let mut handles = Vec::new();
        let mut total = 0u8;
        for _ in 0..count {
            handles.push(thread::spawn(move || {
                unsafe {
                    let mut buffer: Vec<u8> = Vec::with_capacity(size);
                    buffer.set_len(size);
                    for i in 0..size {
                        *buffer.get_unchecked_mut(i) = 42;
                    }
                    buffer.iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
                }
            }));
        }
        for handle in handles {
            total = total.wrapping_add(handle.join().unwrap());
        }
        Ok(total)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let count: usize = args.get(1).unwrap_or(&"1000".to_string()).parse().unwrap();
    let size: usize = args.get(2).unwrap_or(&"1048576".to_string()).parse().unwrap();
    match ResourceManager::process_request(count, size) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

/*
The test oracle below verifies that the corrected version safely rejects resource requests that exceed limits.
When executing process_request with parameters that exceed safe thresholds (e.g., too many threads or a too-large allocation),
the fixed code should return an error, while the vulnerable version would proceed with the allocation.
This oracle function is used in a unit test environment.
*/
#[cfg(test)]
mod tests {
    use super::ResourceManager;
    
    #[test]
    fn resource_allocation_limit_test() {
        // Triggering input that exceeds safe limits.
        let excessive_count = 2000;            // Exceeds MAX_THREADS (1000).
        let excessive_size = 20 * 1024 * 1024;   // Exceeds MAX_ALLOCATION (10 MB).
        let result = ResourceManager::process_request(excessive_count, excessive_size);
        // Expect an error for safe (fixed) version.
        assert!(result.is_err(), "Expected safe limits to trigger an error");
    }
}
