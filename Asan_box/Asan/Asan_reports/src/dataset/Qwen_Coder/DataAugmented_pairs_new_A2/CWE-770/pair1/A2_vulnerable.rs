use std::env;
use std::thread;

pub struct ResourceManager;

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager
    }

    pub fn process(&self, alloc: usize) -> Result<u64, &'static str> {
        let handle = thread::spawn(move || -> u64 {
            let mut buffer: Vec<u8> = Vec::with_capacity(alloc);
            unsafe {
                buffer.set_len(alloc);
            }
            for i in 0..alloc {
                buffer[i] = 42;
            }
            buffer.iter().map(|&b| b as u64).sum()
        });
        handle.join().map_err(|_| "Thread panicked")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: <binary> <allocation_size>");
        return;
    }
    let alloc: usize = args[1].parse().unwrap_or(0);
    let mgr = ResourceManager::new();
    match mgr.process(alloc) {
        Ok(sum) => println!("Sum: {}", sum),
        Err(err) => println!("Error: {}", err),
    }
}

//////////////////////////////////////////////
// Test Oracle Function
// This test verifies that the allocation size limit is enforced.
// In the corrected version, a request above the limit produces an Err result.
// In the original version, such a request might succeed (or even panic).
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::ResourceManager;

    #[test]
    fn allocation_limit_enforcement() {
        let mgr = ResourceManager::new();
        // Request an allocation size that exceeds the permitted maximum.
        let excessive_allocation = 2_000_000;
        let result = mgr.process(excessive_allocation);
        // The test asserts that the result is an error.
        // The vulnerable version would not have enforced a limit and hence might succeed.
        assert!(result.is_err(), "Allocation size should be limited by the fix");
    }
}
