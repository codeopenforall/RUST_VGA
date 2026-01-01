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

#[cfg(test)]
mod tests {
    use super::execute;

    // Oracle function: for an input exceeding the allowed MAX_TASKS, the fixed version should return an error,
    // whereas the vulnerable version would (incorrectly) proceed and return Ok.
    #[test]
    fn test_resource_throttling() {
        // Use a value that exceeds the limit (MAX_TASKS is 100 in the fixed version)
        let result = execute(150);
        // The fixed version should error out. If it returns Ok, then it's vulnerable.
        assert!(result.is_err(), "Expected error due to too many tasks, but got Ok");
    }
}
