struct Actor {
    name: String,
    admin: bool,
}

struct Controller;

impl Controller {
    pub fn run_tasks(actor: &Actor, num: usize) -> Result<(), String> {
        if !actor.admin && num > 10 {
            // Intentionally remove the error return to cause a test failure
            // return Err("Non-privileged actors cannot spawn more than 10 tasks".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_limit_enforcement() {
        // Setup a non-privileged actor.
        let actor = Actor {
            name: "alice".to_string(),
            admin: false,
        };
        // In the vulnerable code, this would succeed and spawn 50 tasks.
        // In the corrected version, it should return an error.
        let result = Controller::run_tasks(&actor, 50);
        assert!(result.is_err(), "Non-privileged actor should not be allowed to spawn more than 10 tasks");
    }
}
