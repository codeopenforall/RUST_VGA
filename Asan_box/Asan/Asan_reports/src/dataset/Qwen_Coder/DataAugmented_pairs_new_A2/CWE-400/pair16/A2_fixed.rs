use std::thread;
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::env;
const MAX_LIMIT: u32 = 1000; 
fn process(limit: u32) -> Result<u64, &'static str> {
    if limit > MAX_LIMIT {
        return Err("Limit exceeds safe threshold");
    }
    let total = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];
    for i in 0..limit {
        let total = total.clone();
        let handle = thread::spawn(move || {
            let local = i as u64;
            let local = local.wrapping_add(1);
            total.fetch_add(local, Ordering::Relaxed);
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().map_err(|_| "Thread join error")?;
    }
    Ok(total.load(Ordering::Relaxed))
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let limit: u32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        10  
    };
    match process(limit) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

/*
The following test oracle function is designed to be compiled and run separately from both the vulnerable and fixed programs.
It calls the process function with a value that exceeds the safe threshold.
For the vulnerable implementation, this test is expected to trigger uncontrolled resource consumption (or even hang),
causing the test to fail (or time out). For the fixed version, the function should return an error immediately.
*/
#[cfg(test)]
mod tests {
    // In order to test the process function, we bring it into scope. 
    // Note: Run this test with the corresponding implementation (vulnerable or fixed).
    use super::process;
    
    #[test]
    fn test_oracle() {
        // Use a limit that is above the safe threshold of the fixed version.
        let result = process(1500);
        // For the fixed version, an error is expected due to the limit check.
        // For the vulnerable version, the test may hang or yield a valid result (indicating a flaw).
        assert!(result.is_err(), "Expected an error when the limit exceeds safe threshold");
    }
}
