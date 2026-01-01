use std::sync::{Arc, Mutex};
use std::thread;

trait Processor {
    fn run(&self, value: i32) -> Result<i32, String>;
}

struct Handler {
    data: Arc<Mutex<i32>>,
}

impl Processor for Handler {
    fn run(&self, value: i32) -> Result<i32, String> {
        if value < 0 {
            Err("Simulated error: negative value".to_string())
        } else {
            Ok(value)
        }
    }
}

fn execute_task(value: i32) -> Result<i32, String> {
    let handler = Handler {
        data: Arc::new(Mutex::new(value)),
    };
    let shared_handler = Arc::new(handler);
    let handler_thread = shared_handler.clone();
    let join_handle = thread::spawn(move || {
        handler_thread.run(value)
    });
    join_handle.join().unwrap()
}

fn main() {
    let output = execute_task(-1);
    match output {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}