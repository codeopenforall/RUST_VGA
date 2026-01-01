use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Duration;

struct Processor {
    value: Arc<AtomicI32>,
}

impl Processor {
    pub fn execute(&self) -> Result<i32, &'static str> {
        let (tx, rx) = mpsc::channel();

        // Simulate an asynchronous task that takes longer than the timeout.
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(2));
            tx.send(()).unwrap();
        });

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(_) => Ok(self.value.load(Ordering::SeqCst)),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(0), // Intentional vulnerability
            Err(mpsc::RecvTimeoutError::Disconnected) => Err("Channel disconnected"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI32, Ordering};

    // The test is designed such that the asynchronous task takes longer than the timeout.
    // Therefore, a correct implementation should return an error.
    #[test]
    fn test_timeout_condition() {
        let processor = Processor { value: Arc::new(AtomicI32::new(42)) };
        let outcome = processor.execute();
        
        // Assert that the outcome is an error.
        assert!(outcome.is_err(), "Expected error due to timeout, but got: {:?}", outcome);
        
        if let Err(msg) = outcome {
            assert_eq!(msg, "Operation timed out", "Error message mismatch");
        }
    }
}
