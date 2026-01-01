struct Task {
    id: u32,
    data: u8,
}

fn run_service(n: u32) -> Result<Vec<Task>, &'static str> {
    const MAX_REQUESTS: u32 = 150;
    if n > MAX_REQUESTS {
        return Err("limit exceeded");
    }
    let mut tasks: Vec<Task> = Vec::with_capacity(n as usize);
    for i in 0..n {
        tasks.push(Task { id: i, data: (i % 256) as u8 });
    }
    Ok(tasks)
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