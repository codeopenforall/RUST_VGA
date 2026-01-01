fn run(trigger: usize) -> usize {
    let mut queue = Vec::new();
    for _ in 0..trigger {
        queue.push(()); // Intentionally push without checking the queue length
    }
    queue.len()
}