use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Duration;

struct Processor {
    value: Arc<AtomicI32>,
}

impl Processor {
    pub fn execute(&self) -> Result<i32, &'static str> {
        let (tx, rx) = mpsc::channel();

        let value_clone = self.value.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2)); // Simulate a long-running task
            tx.send(value_clone.load(Ordering::SeqCst)).unwrap();
        });

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(value) => Ok(value),
            Err(mpsc::RecvTimeoutError::Timeout) => Err("Operation timed out"),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err("Channel disconnected"),
        }
    }
}