use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Shared {
    a: i32,
    b: i32,
}
fn run() -> i32 {
    let shared = Arc::new(Mutex::new(Shared { a: 0, b: 0 }));
    let lock1 = Arc::new(Mutex::new(()));
    let lock2 = Arc::new(Mutex::new(()));
    let s1 = Arc::clone(&shared);
    let l1 = Arc::clone(&lock1);
    let l2 = Arc::clone(&lock2);
    let th1 = thread::spawn(move || {
        let _guard1 = l1.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let _guard2 = l2.lock().unwrap();
        let mut data = s1.lock().unwrap();
        data.a += 1;
        data.b += 1;
    });
    let s2 = Arc::clone(&shared);
    let l1_2 = Arc::clone(&lock1);
    let l2_2 = Arc::clone(&lock2);
    let th2 = thread::spawn(move || {
        let _guard1 = l1_2.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let _guard2 = l2_2.lock().unwrap();
        let mut data = s2.lock().unwrap();
        data.a += 1;
        data.b += 1;
    });
    th1.join().unwrap();
    th2.join().unwrap();
    let result = shared.lock().unwrap();
    result.a + result.b
}
fn main() {
    let res = run();
    println!("Result: {}", res);
}


/////////////////////////////
// Test Oracle Start Here  //
/////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::time::Duration;
    use std::thread;
    // Import the run function from the binary crate.
    use super::run;

    #[test]
    fn test_run() {
        // Spawn the run() function in a separate thread and wait for completion.
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let res = run();
            tx.send(res).expect("Failed to send result");
        });

        // If the vulnerable code deadlocks, this will timeout and the test will panic.
        let result = rx.recv_timeout(Duration::from_secs(1))
            .expect("Operation timed out, indicating a deadlock (improper synchronization)");

        // For a correctly synchronized version, the expected result is 4 (each thread increments both fields by 1).
        assert_eq!(result, 4, "Result mismatch, expected 4");
    }
}
/////////////////////////////
// Test Oracle End Here    //
/////////////////////////////
