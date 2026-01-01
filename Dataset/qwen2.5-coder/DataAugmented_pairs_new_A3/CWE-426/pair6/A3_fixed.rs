use std::process::Command;
use std::sync::Arc;
use std::thread;

trait Runner {
    fn run(&self) -> bool;
}

struct Handler {
    exec_path: Arc<String>,
}

impl Runner for Handler {
    fn run(&self) -> bool {
        // Remove the unsafe block as it's not needed here
        match Command::new(&*self.exec_path).output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}

fn spawn_task() -> bool {
    // Change the command path to a trusted absolute path
    let cmd = Arc::new(String::from("/bin/true")); // Use /bin/true which always succeeds
    let handler = Handler { exec_path: cmd };
    let handle = thread::spawn(move || handler.run());
    handle.join().unwrap_or(false)
}

fn main() {
    if spawn_task() {
        println!("Operation succeeded");
    } else {
        println!("Operation failed");
    }
}