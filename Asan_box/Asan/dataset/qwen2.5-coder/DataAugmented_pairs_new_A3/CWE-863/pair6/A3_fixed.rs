fn run_tasks(n: usize) -> Result<usize, &'static str> {
    const MAX_TASKS: usize = 100;
    if n > MAX_TASKS {
        return Err("Task limit exceeded");
    }
    let num_tasks = n;
    let sum: usize = (0..num_tasks).sum();
    Ok(sum)
}