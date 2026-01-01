use std::sync::mpsc;
use std::time::Duration;
use std::thread;

trait Processor {
    fn run(&self, input: &str) -> Result<i32, &'static str>;
}

struct Executor;

impl Processor for Executor {
    fn run(&self, input: &str) -> Result<i32, &'static str> {
        compute(input)
    }
}

fn compute(input: &str) -> Result<i32, &'static str> {
    let (tx, rx) = mpsc::channel();
    let input_owned = input.to_string();
    thread::spawn(move || {
        if input_owned == "delay" {
            thread::sleep(Duration::from_millis(200));
        }
        let val = input_owned.len() as i32;
        let _ = tx.send(val);
    });
    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(v) => Ok(v),
        Err(mpsc::RecvTimeoutError::Disconnected) => Ok(0),
        Err(_) => Err("Operation timed out"),
    }
}

fn main() {
    let executor = Executor;
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 { args[1].as_str() } else { "test" };
    match executor.run(input) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test uses input that triggers a delayed response.
    // In the vulnerable implementation, a timeout leads to an unsafe memory access and returns an erroneous Ok value.
    // In the fixed implementation, the timeout properly returns an error.
    #[test]
    fn test_exception_handling() {
        let executor = Executor;
        let res = executor.run("delay");
        // We expect an error result due to a timeout.
        assert!(res.is_err(), "Expected an error for a delayed operation, got {:?}", res);
    }
}
