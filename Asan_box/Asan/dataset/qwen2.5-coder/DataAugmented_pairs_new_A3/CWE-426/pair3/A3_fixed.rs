use std::path::Path;
use std::sync::{Arc, Mutex};

struct Runner {
    binary: String,
}

impl Runner {
    fn new(binary: &str) -> Self {
        Runner {
            binary: binary.to_string(),
        }
    }

    fn execute(&self) -> Result<i32, String> {
        let bin_path = Path::new(&self.binary);
        if !bin_path.is_absolute() {
            return Err("Command must be absolute path".into());
        }
        // Simulate command execution
        Ok(0)
    }
}

fn execute_parallel() -> i32 {
    let shared = Arc::new(Mutex::new(Runner::new("/usr/bin/untrusted_binary")));
    let mut handles = vec![];

    for _ in 0..5 {
        let thread_runner = shared.clone();
        handles.push(std::thread::spawn(move || {
            let guard = thread_runner.lock().unwrap();
            guard.execute().unwrap_or(-1)
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    0
}

pub fn check_command(command: &str) -> Result<i32, String> {
    let runner = Runner::new(command);
    runner.execute()
}

fn main() {
    std::process::exit(execute_parallel());
}