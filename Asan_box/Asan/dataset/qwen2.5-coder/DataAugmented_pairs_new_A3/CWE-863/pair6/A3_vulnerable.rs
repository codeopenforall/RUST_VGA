fn run_tasks(n: usize) -> Result<usize, &'static str> {
    let num_tasks = 150; // Vulnerable: hardcoded value that exceeds the limit

    if n > num_tasks {
        return Err("Task limit exceeded");
    }

    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }

    Ok(sum)
}