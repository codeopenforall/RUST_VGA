use std::thread;
const MAX_REQUESTS: u32 = 150;
pub struct Task {
    pub id: u32,
    pub data: u8,
}
pub fn run_service(n: u32) -> Result<(), &'static str> {
    if n > MAX_REQUESTS {
        return Err("limit exceeded");
    }
    let mut tasks: Vec<Task> = Vec::with_capacity(n as usize);
    for i in 0..n {
        tasks.push(Task { id: i, data: (i % 256) as u8 });
    }
    let mut handles = Vec::new();
    for task in tasks {
        let handle = thread::spawn(move || {
            let mut sum: u32 = 0;
            for _ in 0..1000 {
                sum = sum.wrapping_add(task.data as u32);
            }
            sum
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
    Ok(())
}
fn main() {
    let input = 200;
    match run_service(input) {
        Ok(_) => println!("Processing complete with input: {}", input),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::run_service;

    #[test]
    fn test_resource_limit() {
        // The input value exceeds the safe threshold.
        let input = 200;
        let result = run_service(input);
        // For the fixed implementation, the function should return an error.
        // In the vulnerable code this test would incorrectly return Ok(_), causing the test to fail.
        assert_eq!(result, Err("limit exceeded"), "Expected error for input exceeding the limit");
    }
}
