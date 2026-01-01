use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Procedure {
    compute_delay: u64,
    timeout: u64,
}

impl Procedure {
    pub fn process(&self) -> Result<u32, &'static str> {
        let (tx, rx) = mpsc::channel();

        thread::spawn({
            let tx = tx.clone();
            let delay = self.compute_delay;
            move || {
                thread::sleep(Duration::from_millis(delay));
                tx.send(42).unwrap();
            }
        });

        match rx.recv_timeout(Duration::from_millis(self.timeout)) {
            Ok(result) => Ok(result),
            Err(mpsc::RecvTimeoutError::Timeout) => Err("Operation timed out"),
            Err(_) => Err("Operation failed unexpectedly"),
        }
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
