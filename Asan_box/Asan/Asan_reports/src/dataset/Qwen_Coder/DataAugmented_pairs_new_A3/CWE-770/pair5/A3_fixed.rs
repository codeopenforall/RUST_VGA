use std::alloc::{alloc, Layout};
use std::env;
use std::sync::Arc;
use std::thread;

trait Processor {
    fn run(&self, count: usize) -> Result<(), &'static str>;
}

struct ResourceManager;

impl Processor for ResourceManager {
    fn run(&self, count: usize) -> Result<(), &'static str> {
        // Add a reasonable limit to prevent excessive memory allocation
        const MAX_COUNT: usize = 1_000_000;
        if count > MAX_COUNT {
            return Err("Count exceeds maximum allowed");
        }

        let layout = Layout::array::<u8>(count).map_err(|_| "Layout error")?;
        unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                return Err("Allocation failed");
            }
            for i in 0..count {
                ptr.add(i).write(42);
            }
            let vec = Vec::from_raw_parts(ptr, count, count);
            let shared = Arc::new(vec);
            let mut workers = vec![];
            for _ in 0..4 {
                let data = Arc::clone(&shared);
                workers.push(thread::spawn(move || {
                    let _sum: u8 = data.iter().fold(0, |acc, &x| acc.wrapping_add(x));
                }));
            }
            for worker in workers {
                worker.join().unwrap();
            }
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(100)
    } else {
        100
    };
    let manager = ResourceManager;
    if let Err(e) = manager.run(count) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Test oracle: call run() with an excessive count.
    // The secure variant should return an error, while the insecure variant would likely succeed (or exhaust resources).
    #[test]
    fn test_allocation_throttle() {
        // Use a test count that exceeds the safe limit imposed in the secure version.
        let excessive = 10_000_000;
        let manager = ResourceManager;
        let result = manager.run(excessive);
        // In the secure variant, the request should be rejected.
        assert!(result.is_err(), "Resource allocation should be throttled for excessive sizes");
    }
}
