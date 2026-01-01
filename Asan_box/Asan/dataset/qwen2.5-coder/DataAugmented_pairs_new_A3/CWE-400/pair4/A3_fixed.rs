struct Service {
    tasks: Vec<String>,
}

impl Service {
    pub fn new() -> Self {
        Service { tasks: Vec::new() }
    }

    pub fn submit(&mut self, task: String) -> Result<(), String> {
        const MAX_TASKS: usize = 1000;
        if self.tasks.len() >= MAX_TASKS {
            return Err("Queue limit reached".to_string());
        }
        self.tasks.push(task);
        println!("Tasks submission complete (max limit enforced)");
        Ok(())
    }
}