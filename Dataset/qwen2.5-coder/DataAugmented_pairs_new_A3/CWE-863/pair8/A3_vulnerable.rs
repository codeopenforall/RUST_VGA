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