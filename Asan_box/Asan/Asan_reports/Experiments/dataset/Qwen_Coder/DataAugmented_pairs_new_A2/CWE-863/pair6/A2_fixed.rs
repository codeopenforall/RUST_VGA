use std::alloc;
use std::thread;
const MAX_TASKS: usize = 100;
pub fn run_tasks(n: usize) -> Result<usize, &'static str> {
    if n > MAX_TASKS {
        return Err("Task limit exceeded");
    }
    unsafe {
        let layout = alloc::Layout::array::<usize>(n)
            .map_err(|_| "Layout error")?;
        let ptr = alloc::alloc(layout) as *mut usize;
        if ptr.is_null() {
            return Err("Allocation failed");
        }
        for i in 0..n {
            *ptr.add(i) = i;
        }
        alloc::dealloc(ptr as *mut u8, layout);
    }
    let mut handles = Vec::with_capacity(n);
    for i in 0..n {
        let handle = thread::spawn(move || {
            i
        });
        handles.push(handle);
    }
    let mut total = 0;
    for h in handles {
        total += h.join().map_err(|_| "Thread panicked")?;
    }
    Ok(total)
}
fn main() {
    let num_tasks = 50;
    match run_tasks(num_tasks) {
        Ok(sum) => println!("Sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::run_tasks;

    #[test]
    fn test_resource_limit() {
        // This input exceeds the allowed threshold.
        let high_input = 150;
        match run_tasks(high_input) {
            // The vulnerable version would process this without error,
            // but the fixed version should return an error.
            Ok(_) => panic!("Test failed: Resource limit was not enforced"),
            Err(e) => assert_eq!(e, "Task limit exceeded"),
        }
    }

    #[test]
    fn test_normal_operation() {
        let safe_input = 50;
        // Both versions should produce a valid sum if input is within limits.
        let result = run_tasks(safe_input);
        assert!(result.is_ok());
        // Compute expected sum = 0 + 1 + ... + (n-1)
        let expected: usize = (0..safe_input).sum();
        assert_eq!(result.unwrap(), expected);
    }
}
