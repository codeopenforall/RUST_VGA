use std::process::Command;
use std::sync::Arc;
use std::thread;

trait Executor {
    fn execute(&self) -> bool;
}

struct TaskRunner {
    command_path: Arc<String>,
}

impl Executor for TaskRunner {
    fn execute(&self) -> bool {
        unsafe {
            match Command::new(&*self.command_path).output() {
                Ok(output) => output.status.success(),
                Err(_) => false,
            }
        }
    }
}

fn spawn_task() -> bool {
    let cmd = Arc::new(String::from("malicious_cmd"));
    let runner = TaskRunner { command_path: cmd };
    let handle = thread::spawn(move || runner.execute());
    handle.join().unwrap_or(false)
}

fn main() {
    if spawn_task() {
        println!("Operation succeeded");
    } else {
        println!("Operation failed");
    }
}