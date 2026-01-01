use std::sync::{mpsc, Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;
use std::ptr;
const LIMIT: usize = 100;
fn unsafe_transform(x: u32) -> u32 {
    unsafe {
        let ptr = &x as *const u32;
        ptr::read(ptr)
    }
}
pub fn run_simulation(iterations: usize) -> Result<usize, String> {
    let (tx, rx) = mpsc::sync_channel(LIMIT);
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    for thread_index in 0..4 {
        let thread_tx = tx.clone();
        let thread_counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for j in 0..iterations {
                let msg = unsafe_transform(j as u32);
                loop {
                    let load = thread_counter.load(Ordering::Relaxed);
                    if load < LIMIT {
                        thread_counter.fetch_add(1, Ordering::Relaxed);
                        break;
                    }
                    thread::sleep(Duration::from_millis(1));
                }
                thread_tx.send(msg).unwrap();
            }
        }));
    }
    drop(tx); 
    let mut processed = 0;
    while let Ok(msg) = rx.recv() {
         let _ = unsafe_transform(msg);
         processed += 1;
         thread::sleep(Duration::from_millis(1));
         counter.fetch_sub(1, Ordering::Relaxed);
    }
    for h in handles {
         let _ = h.join();
    }
    Ok(processed)
}
fn main() {
    match run_simulation(1000) {
         Ok(n) => println!("Processed {} messages", n),
         Err(e) => eprintln!("Error: {}", e),
    }
}

//////////////////////////////////////////
// Test Oracle Function
//
// Description:
// This integration test (designed for use as a unit test) calls the run_simulation()
// function with a specific iteration count (1000 per producer thread). In the vulnerable
// version, the function is expected to panic (or otherwise fail) due to exceeding the
// resource limit, whereas the fixed version should process exactly 4000 messages (4 threads x 1000 iterations).
//
// The test asserts that in the fixed version the function returns Ok(4000),
// and in the vulnerable version it panics or returns an error, causing the test to fail.
//////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::panic;
    use super::run_simulation;

    #[test]
    fn integration_test() {
         // Catch unwinding to determine if the function panics.
         let result = panic::catch_unwind(|| {
             run_simulation(1000)
         });
         match result {
             Ok(Ok(count)) => {
                 // The fixed version should process exactly 4000 messages.
                 assert_eq!(count, 4000, "Incorrect number of processed messages.");
             }
             Ok(Err(e)) => {
                 panic!("Function returned an error: {}", e);
             }
             Err(_) => {
                 panic!("Function panicked, indicating uncontrolled resource consumption vulnerability.");
             }
         }
    }
}
