use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Shared {
    lock1: Mutex<u32>,
    lock2: Mutex<u32>,
}
fn main() {
    let shared = Arc::new(Shared {
        lock1: Mutex::new(0),
        lock2: Mutex::new(0),
    });
    let s1 = shared.clone();
    let t1 = thread::spawn(move || {
        let mut g1 = s1.lock1.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        let mut g2 = s1.lock2.lock().unwrap();
        *g1 += 1;
        *g2 += 1;
    });
    let s2 = shared.clone();
    let t2 = thread::spawn(move || {
        let mut g1 = s2.lock1.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        let mut g2 = s2.lock2.lock().unwrap();
        *g1 += 1;
        *g2 += 1;
    });
    t1.join().unwrap();
    t2.join().unwrap();
    let v1 = *shared.lock1.lock().unwrap();
    let v2 = *shared.lock2.lock().unwrap();
    println!("Results: {} and {}", v1, v2);
}


#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    // This test verifies that the application's main routine completes within a reasonable time.
    // The vulnerable version will likely deadlock or stall, causing the test to fail.
    #[test]
    fn app_completes() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            // Call the main entry point of the application.
            super::main();
            let _ = tx.send(());
        });
        // Expect the main routine to complete within 1 second.
        assert!(
            rx.recv_timeout(Duration::from_secs(1)).is_ok(),
            "The application did not complete in time"
        );
    }
}
