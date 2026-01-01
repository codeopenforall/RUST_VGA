use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Duration;

pub struct DataReader;

impl DataReader {
    pub fn obtain() -> Result<i32, &'static str> {
        let (tx, rx) = mpsc::channel();

        // Simulate a long-running task
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(2));
            tx.send(42).unwrap();
        });

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(value) => Ok(value),
            Err(RecvTimeoutError::Timeout) => return Err("timeout"),
            Err(RecvTimeoutError::Disconnected) => return Err("disconnected"),
        }
    }
}