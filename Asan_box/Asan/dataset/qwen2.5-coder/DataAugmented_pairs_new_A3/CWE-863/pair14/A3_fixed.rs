const MAX_TASKS: usize = 100;

fn execute(num_tasks: usize) -> Result<(), String> {
    if num_tasks > MAX_TASKS {
        return Err("Error: Too many tasks requested".to_string());
    }
    // Simulate task execution
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let num_tasks = args.get(1).map_or(MAX_TASKS, |arg| arg.parse().unwrap_or(MAX_TASKS));

    if let Err(e) = execute(num_tasks) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}