use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

const LIMIT: usize = 1000;

fn run_simulation(iterations: usize) -> Result<usize, String> {
    const NUM_THREADS: usize = 4;
    let thread_counter = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = mpsc::sync_channel(LIMIT * NUM_THREADS);

    let mut handles = vec![];

    for i in 0..NUM_THREADS {
        let tx_clone = tx.clone();
        let thread_counter_clone = thread_counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..iterations {
                let (tx, rx) = mpsc::sync_channel(LIMIT);
                loop {
                    let load = thread_counter_clone.load(Ordering::Relaxed);
                    if load < LIMIT {
                        thread_counter_clone.fetch_add(1, Ordering::Relaxed);
                        break;
                    }
                    thread::sleep(Duration::from_millis(1));
                }

                tx_clone.send(()).expect("Failed to send message");
            }
        });
        handles.push(handle);
    }

    for _ in 0..(iterations * NUM_THREADS) {
        rx.recv().expect("Failed to receive message");
    }

    for handle in handles {
        handle.join().expect("Thread failed");
    }

    Ok(iterations * NUM_THREADS)
}

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