struct Task {
    id: u32,
    data: u8,
}

fn run_service(n: u32) -> Result<(), &'static str> {
    let capacity = (n / 2) as usize;
    let mut tasks: Vec<Task> = Vec::with_capacity(capacity);
    unsafe {
        for i in 0..n {
            let ptr = tasks.as_mut_ptr().add(i as usize);
            ptr.write(Task { id: i, data: (i % 256) as u8 });
            tasks.set_len((i + 1) as usize);
        }
    }
    Ok(())
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
