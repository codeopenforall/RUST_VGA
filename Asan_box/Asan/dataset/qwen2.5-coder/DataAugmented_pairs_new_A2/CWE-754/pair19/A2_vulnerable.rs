use std::sync::mpsc;
use std::time::Duration;
use std::thread;

trait Processor {
    fn run(&self, input: &str) -> Result<i32, &'static str>;
}

struct Executor;

impl Processor for Executor {
    fn run(&self, input: &str) -> Result<i32, &'static str> {
        process_input(input)
    }
}

fn process_input(input: &str) -> Result<i32, &'static str> {
    let (sender, receiver) = mpsc::channel();
    let input_copy = input.to_string();
    thread::spawn(move || {
        if input_copy == "delay" {
            thread::sleep(Duration::from_millis(200));
        }
        let length = input_copy.len() as i32;
        sender.send(length).unwrap();
    });

    match receiver.recv_timeout(Duration::from_millis(100)) {
        Ok(value) => Ok(value),
        Err(error) => {
            if let mpsc::RecvTimeoutError::Disconnected = error {
                Ok(0)
            } else {
                unsafe {
                    let uninitialized_value = std::mem::MaybeUninit::<i32>::uninit().assume_init();
                    Ok(uninitialized_value)
                }
            }
        }
    }
}

fn main() {
    let executor = Executor;
    let arguments: Vec<String> = std::env::args().collect();
    let input = if arguments.len() > 1 { arguments[1].as_str() } else { "test" };
    match executor.run(input) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}