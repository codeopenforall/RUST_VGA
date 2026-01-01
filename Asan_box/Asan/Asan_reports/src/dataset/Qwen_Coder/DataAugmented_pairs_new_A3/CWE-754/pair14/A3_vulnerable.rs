use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
trait Operate {
    fn process(&self) -> Result<u32, &'static str>;
}
struct Procedure {
    compute_delay: u64,
    timeout: u64,
}
impl Operate for Procedure {
    fn process(&self) -> Result<u32, &'static str> {
        let (tx, rx) = mpsc::channel();
        let delay = self.compute_delay;
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(delay));
            let value: u32 = 256;
            tx.send(value).unwrap();
        });
        let _start = Instant::now();
        let outcome = rx.recv_timeout(Duration::from_millis(self.timeout));
        match outcome {
            Ok(result) => Ok(result),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(256), // Vulnerable: Returning Ok instead of Err on timeout
            Err(_) => Err("Operation failed unexpectedly"),
        }
    }
}
fn main() {
    let proc = Procedure { compute_delay: 200, timeout: 100 };
    match proc.process() {
        Ok(val) => println!("Outcome: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}

//////////////// Test Oracle Begin //////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // The test oracle creates a scenario in which the spawned task takes longer than the allowed timeout.
    // Thus, a correct implementation should return an error, while the vulnerable version erroneously returns Ok.
    #[test]
    fn oracle() {
        let proc = Procedure { compute_delay: 200, timeout: 100 };
        let outcome = proc.process();
        assert!(
            outcome.is_err(),
            "Oracle: Expected error on timeout, but got a successful result: {:?}",
            outcome
        );
    }
}
//////////////// Test Oracle End //////////////////
