use std::panic;
use std::thread;

struct Service;

impl Service {
    fn execute(&self, trigger: bool) -> Result<(), &'static str> {
        let handle = thread::spawn(move || {
            let result = panic::catch_unwind(|| {
                if trigger {
                    panic!("Triggered panic");
                }
            });
            result
        });

        match handle.join() {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(_)) => Err("Thread encountered a panic"),
            Err(_) => Err("Failed to join thread"),
        }
    }
}

fn main() {
    let service = Service;
    match service.execute(true) {
        Ok(_) => println!("Execution completed successfully."),
        Err(e) => println!("Execution error handled: {}", e),
    }
}