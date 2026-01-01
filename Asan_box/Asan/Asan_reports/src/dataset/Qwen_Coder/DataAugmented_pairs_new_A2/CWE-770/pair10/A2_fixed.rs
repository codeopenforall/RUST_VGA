use std::env;
use std::sync::Arc;
use std::thread;
const MAX_CAP: usize = 1000; 
trait Processor {
    fn process(&self, count: usize) -> Result<u64, String>;
}
struct Manager;
impl Manager {
    fn allocate(&self, size: usize) -> Result<Vec<u8>, String> {
        if size > MAX_CAP {
            return Err(format!(
                "Requested size {} exceeds the maximum allowed {}",
                size, MAX_CAP
            ));
        }
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 1); 
        Ok(vec)
    }
}
impl Processor for Manager {
    fn process(&self, count: usize) -> Result<u64, String> {
        let data = self.allocate(count)?;
        let chunk_size = if count < 4 { 1 } else { count / 4 };
        let num_threads = if count < 4 { 1 } else { 4 };
        let shared_data = Arc::new(data);
        let mut handles = Vec::new();
        let mut total: u64 = 0;
        for idx in 0..num_threads {
            let cloned_data = Arc::clone(&shared_data);
            let handle = thread::spawn(move || -> u64 {
                let start = idx * chunk_size;
                let end = if idx == num_threads - 1 {
                    cloned_data.len()
                } else {
                    start + chunk_size
                };
                let mut local_sum = 0u64;
                for i in start..end {
                    local_sum += cloned_data[i] as u64;
                }
                local_sum
            });
            handles.push(handle);
        }
        for handle in handles {
            match handle.join() {
                Ok(val) => total += val,
                Err(_) => return Err("A thread panicked".to_string()),
            }
        }
        Ok(total)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };
    let instance = Manager;
    match instance.process(count) {
        Ok(result) => println!("Final Result: {}", result),
        Err(e) => eprintln!("Error occurred: {}", e),
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
