use std::env;
use std::thread;

fn allocate_resources(count: usize) -> Result<(), String> {
    let mut handles = Vec::with_capacity(count);
    for i in 0..count {
        let handle = thread::spawn(move || {
            unsafe {
                let data = [1u8; 10];
                let ptr = data.as_ptr().offset(i as isize);
                let _ = *ptr;
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().map_err(|_| "Thread panicked".to_string())?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    match allocate_resources(count) {
        Ok(_) => println!("Completed allocation with count {}", count),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    use super::allocate_resources;

    #[test]
    fn test_allocation() {
        // Use a count value higher than allowed to trigger the vulnerability/fix.
        let count = 15;
        // For the vulnerable code, this input is expected to cause undefined behavior (likely a panic).
        // For the fixed code, the function should safely return an error.
        let result = panic::catch_unwind(|| allocate_resources(count));

        match result {
            Err(_) => panic!("Test failed: function panicked, indicating the unsafe resource allocation vulnerability."),
            Ok(res) => {
                // The fixed version is expected to return an error when excessive resources are requested.
                assert!(res.is_err(), "Expected an error due to resource allocation limits; got Ok instead.");
            }
        }
    }
}
