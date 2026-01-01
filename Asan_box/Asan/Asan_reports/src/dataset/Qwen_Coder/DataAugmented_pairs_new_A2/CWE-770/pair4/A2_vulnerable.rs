use std::env;
use std::thread::{self, JoinHandle};

pub fn allocate_resources(n: usize) -> Result<u32, String> {
    let mut handles: Vec<JoinHandle<u32>> = Vec::new();
    unsafe {
        let ptr = handles.as_mut_ptr();
        for i in 0..n {
            ptr.add(i).write(thread::spawn(move || i as u32));
        }
        handles.set_len(n);
    }
    let mut sum: u32 = 0;
    for handle in handles {
        sum = sum.saturating_add(handle.join().map_err(|_| "thread join error".to_string())?);
    }
    Ok(sum)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };
    match allocate_resources(input) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::allocate_resources;
    
    #[test]
    fn test_resource_cap() {
        // Using a value above the allowed threshold.
        let excessive_input = 101;
        let result = allocate_resources(excessive_input);
        // For the fixed version, we expect an error to be returned.
        // For the vulnerable version, this function is expected to behave unsafely (potential crash or undefined behavior).
        assert!(result.is_err(), "The function must return an error when input exceeds the limit");
    }
}
