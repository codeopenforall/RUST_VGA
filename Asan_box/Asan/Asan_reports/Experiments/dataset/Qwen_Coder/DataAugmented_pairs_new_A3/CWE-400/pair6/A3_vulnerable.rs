use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::sync::mpsc;

const LIMIT: usize = 1000;

fn run_simulation(iterations: usize) -> Result<usize, String> {
    let thread_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for thread_index in 0..4 {
        let thread_counter_clone = thread_counter.clone();
        let (tx, rx) = mpsc::channel(); 

        let handle = thread::spawn(move || {
            let load = thread_counter_clone.fetch_add(1, Ordering::Relaxed) + 1;
            if load > LIMIT {
                panic!("Resource limit exceeded in producer thread {}", thread_index);
            }

            for _ in 0..iterations {
                tx.send(()).unwrap();
            }
        });

        handles.push(handle);
    }

    let mut count = 0;
    for _ in 0..(4 * iterations) {
        rx.recv().unwrap();
        count += 1;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_test() {
        let result = run_simulation(1000);
        assert!(result.is_err(), "Test should fail due to resource limit exceeded.");
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
