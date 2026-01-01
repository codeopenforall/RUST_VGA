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