use std::env;
use std::thread;
use std::ptr;
use std::sync::{Arc, Mutex};

struct ResourceManager;

impl ResourceManager {
    fn run_tasks(&self, count: usize) -> Result<usize, &'static str> {
        if count > 200 {
            return Err("Excessive resource request");
        }

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for i in 0..count {
            let counter_clone = Arc::clone(&counter);
            unsafe {
                let dummy = i as *const usize;
                let _ = ptr::read(dummy);
            }
            let handle = thread::spawn(move || {
                {
                    let mut num = counter_clone.lock().unwrap();
                    *num += 1;
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().map_err(|_| "Thread panicked")?;
        }
        let final_count = *counter.lock().unwrap();
        Ok(final_count)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(100)
    } else {
        100
    };
    let manager = ResourceManager;
    match manager.run_tasks(count) {
        Ok(n) => println!("Completed {} tasks.", n),
        Err(e) => println!("Error: {}", e),
    }
}

/////////////////////////////////////////////////////////////////
// Test Oracle: This function is executed in a unit test environment.
// It calls the resource allocation function with an input that exceeds the secure limit.
// For the insecure version, the function would spawn 250 threads and return Ok(250),
// causing the test to fail. For the secure version, the function should return an error.
/////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_throttling() {
        let manager = ResourceManager;
        // Use an input that exceeds the allowed maximum.
        let result = manager.run_tasks(250);
        match result {
            Ok(n) => {
                // In the vulnerable version, spawning 250 threads leads to success,
                // which is not acceptable.
                panic!("Test failed: Expected error due to excessive resource request, but got {} tasks", n);
            },
            Err(e) => {
                // In the secure version, an error is rightly returned.
                assert_eq!(e, "Excessive resource request");
            }
        }
    }
}
