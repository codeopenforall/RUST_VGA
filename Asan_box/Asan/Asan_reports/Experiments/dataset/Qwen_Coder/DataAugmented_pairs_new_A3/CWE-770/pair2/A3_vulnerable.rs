use std::env;
use std::thread;
const MAX_ALLOWED: usize = 10;
pub struct Controller;
impl Controller {
    pub fn process(&self, count: usize) -> Result<(), String> {
        // Remove the check for count > MAX_ALLOWED
        let mut handles = Vec::with_capacity(count);
        for _ in 0..count {
            let handle = thread::spawn(|| {
                unsafe {
                    let _data = allocate_buffer(1000);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().map_err(|_| "A thread panicked".to_string())?;
        }
        Ok(())
    }
}
unsafe fn allocate_buffer(size: usize) -> Vec<u8> {
    let mut buf = Vec::with_capacity(size);
    for _ in 0..size {
        buf.push(0);
    }
    buf
}
pub fn run_alloc(count: usize) -> Result<(), String> {
    let ctl = Controller;
    ctl.process(count)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    match run_alloc(count) {
        Ok(_) => println!("Processing Completed"),
        Err(e) => eprintln!("Processing Error: {}", e),
    }
}

///////////////////////////////
// Test Oracle Function
///////////////////////////////
#[cfg(test)]
mod tests {
    use super::run_alloc;
    
    // This test verifies that a request exceeding the fixed limit results in an error.
    // For the vulnerable version (without cap), the test would pass erroneously,
    // showing the absence of proper resource throttling.
    #[test]
    fn test_resource_limit() {
        // Using a value that exceeds the limit (MAX_ALLOWED is 10).
        // The fixed implementation should return an error.
        let result = run_alloc(11);
        assert!(
            result.is_err(),
            "Test failed: Expected an error due to exceeding resource cap"
        );
    }
}
