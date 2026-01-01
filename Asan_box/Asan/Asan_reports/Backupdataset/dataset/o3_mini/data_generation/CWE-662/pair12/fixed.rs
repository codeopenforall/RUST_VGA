#!/usr/bin/env rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Data {
    value: i32,
}
fn run() -> (i32, i32) {
    let res_a = Arc::new(Mutex::new(Data { value: 0 }));
    let res_b = Arc::new(Mutex::new(Data { value: 100 }));
    let a1 = Arc::clone(&res_a);
    let b1 = Arc::clone(&res_b);
    let h1 = thread::spawn(move || {
        let lock_a = a1.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let lock_b = b1.lock().unwrap();
        unsafe {
            let ptr = &*lock_a as *const Data as *mut Data;
            (*ptr).value += 1;
        }
    });
    let a2 = Arc::clone(&res_a);
    let b2 = Arc::clone(&res_b);
    let h2 = thread::spawn(move || {
        let lock_a = a2.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let lock_b = b2.lock().unwrap();
        unsafe {
            let ptr = &*lock_b as *const Data as *mut Data;
            (*ptr).value -= 1;
        }
    });
    h1.join().unwrap();
    h2.join().unwrap();
    let final_a = res_a.lock().unwrap().value;
    let final_b = res_b.lock().unwrap().value;
    (final_a, final_b)
}
fn main() {
    let (a, b) = run();
    println!("Resource A: {}, Resource B: {}", a, b);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;
    use std::thread;

    // Oracle function: For the vulnerable version, the test should time out (simulating deadlock).
    // For the fixed version, the test returns (1, 99).
    // This test assumes that the respective binary being tested calls run() from main.
    #[test]
    fn test_run_behavior() {
        // Use a channel to signal completion of run().
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            // Change the call below to run() from the appropriate binary (vulnerable or fixed)
            let res = run();
            tx.send(res).unwrap();
        });

        // If the vulnerable version deadlocks, no message will be received.
        // Using a timeout to detect the deadlock.
        match rx.recv_timeout(Duration::from_secs(2)) {
            Ok((a, b)) => {
                // The expected correct result for the fixed version.
                assert_eq!(a, 1, "Resource A value mismatch");
                assert_eq!(b, 99, "Resource B value mismatch");
            },
            Err(_) => {
                panic!("Test failed: Execution did not complete within timeout (deadlock suspected).");
            }
        }
    }
}
