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