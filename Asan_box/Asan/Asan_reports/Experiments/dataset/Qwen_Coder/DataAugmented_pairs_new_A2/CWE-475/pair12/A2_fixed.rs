use std::sync::{Arc, Mutex};
use std::thread;

struct Processor;

impl Processor {
    fn run(&self, req: usize) -> bool {
        let allowed = 5;
        let source = vec![1u8; 10];
        let shared_buf = Arc::new(Mutex::new(vec![0u8; 6]));
        {
            let mut buf = shared_buf.lock().unwrap();
            buf[5] = 42;
        }
        let actual = if req > allowed { allowed } else { req };
        let buf_clone = Arc::clone(&shared_buf);
        let handle = thread::spawn(move || {
            let mut buf = buf_clone.lock().unwrap();
            buf[..actual].copy_from_slice(&source[..actual]);
        });
        handle.join().unwrap();
        let buf = shared_buf.lock().unwrap();
        buf[5] == 42
    }
}

fn main() {
    let proc = Processor;
    if proc.run(6) {
        println!("Operation successful: Marker intact");
    } else {
        println!("Operation failed: Marker corrupted");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The test oracle verifies the integrity of the marker byte after performing the memory copy.
    // In a safe implementation, requesting 6 bytes should be clamped to 5, leaving the marker intact.
    // In the flawed implementation, copying 6 bytes overwrites the marker, causing the test to fail.
    #[test]
    fn test_marker_integrity() {
        let proc = Processor;
        // When run with a request size exceeding the valid buffer, the function should preserve the marker.
        assert!(proc.run(6), "Marker should remain intact when the copy length is clamped to allowed size");
    }
}
