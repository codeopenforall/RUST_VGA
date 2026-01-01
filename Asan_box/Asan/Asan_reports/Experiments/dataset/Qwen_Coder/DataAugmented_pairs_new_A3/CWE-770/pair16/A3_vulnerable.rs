use std::alloc::{alloc, dealloc, Layout};
use std::thread;
const MAX_LIMIT: usize = 100; 
fn allocate_resources(count: usize) -> Result<usize, String> {
    // Remove the check for count > MAX_LIMIT
    let allocation = count
        .checked_mul(1_000_000)
        .ok_or("Multiplication overflow")?;
    let layout = Layout::from_size_align(allocation, 8).map_err(|_| "Invalid layout")?;
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            return Err("Allocation failed".into());
        }
        dealloc(ptr, layout);
    }
    Ok(allocation)
}
fn spawn_allocation(count: usize) -> thread::JoinHandle<Result<usize, String>> {
    thread::spawn(move || {
        allocate_resources(count)
    })
}
fn main() {
    let count = 150;
    let handle = spawn_allocation(count);
    match handle.join() {
        Ok(result) => match result {
            Ok(val) => println!("Successfully allocated {} bytes", val),
            Err(e) => println!("Error during allocation: {}", e),
        },
        Err(_) => println!("Thread panicked"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_limit_enforcement() {
        // Using a value above the maximum limit should produce an error.
        let count = 150;
        let handle = spawn_allocation(count);
        let result = handle.join().expect("Thread panicked in test");
        // The fixed version enforces the limit and must return an error.
        assert!(result.is_err(), "Expected error when exceeding resource limit");
    }
}
