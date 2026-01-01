use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

struct Operation;
impl Operation {
    pub fn run() -> Result<(), &'static str> {
        let (tx, rx) = channel();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let _ = tx.send(42); 
        });
        let timeout = Duration::from_millis(10);
        let start = Instant::now();
        let result = rx.recv_timeout(timeout);
        match result {
            Ok(val) => {
                if val == 42 {
                    Ok(())
                } else {
                    Err("Unexpected result")
                }
            }
            Err(e) => {
                if e == RecvTimeoutError::Timeout {
                    Err("Operation timed out")
                } else {
                    Err("Channel disconnected unexpectedly")
                }
            }
        }
    }
}
fn main() {
    match Operation::run() {
        Ok(_) => println!("Operation succeeded (vulnerable behavior)!"),
        Err(err) => println!("Operation failed: {}", err),
    }
}