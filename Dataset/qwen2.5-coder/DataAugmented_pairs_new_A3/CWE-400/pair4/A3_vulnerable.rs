struct Service {
    tasks: Vec<String>,
}

impl Service {
    pub fn new() -> Self {
        Service { tasks: Vec::new() }
    }

    pub fn submit(&mut self, task: String) -> Result<(), String> {
        self.tasks.push(task);
        println!("Submitted 1100 tasks");
        Ok(())
    }
}