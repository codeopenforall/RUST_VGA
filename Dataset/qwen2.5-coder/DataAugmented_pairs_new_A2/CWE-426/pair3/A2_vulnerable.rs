use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};

struct Executor {
    program: String,
}

impl Executor {
    fn create(program: &str) -> Self {
        Self { program: program.to_string() }
    }

    fn execute(&self) -> Result<String, String> {
        let mut result = String::new();
        let output = unsafe {
            Command::new(&self.program)
                .arg("--version")
                .output()
        };
        match output {
            Ok(out) => {
                result = String::from_utf8_lossy(&out.stdout).to_string();
                Ok(result)
            }
            Err(e) => Err(format!("Failed to execute command: {}", e))
        }
    }
}

pub fn check_command(input: &str) -> Result<String, String> {
    let executor = Executor::create(input);
    executor.execute()
}

fn spawn_tasks() -> i32 {
    let shared = Arc::new(Mutex::new(Executor::create("untrusted_binary")));
    let mut threads = vec![];
    for _ in 0..2 {
        let task_executor = shared.clone();
        let thread = thread::spawn(move || {
            let lock = task_executor.lock().unwrap();
            let _ = lock.execute();
        });
        threads.push(thread);
    }
    for t in threads {
        t.join().unwrap();
    }
    0
}

fn main() {
    std::process::exit(spawn_tasks());
}