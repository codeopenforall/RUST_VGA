use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

struct Data {
    counter: Arc<AtomicUsize>,
}

impl Data {
    fn new(initial: usize) -> Self {
        Data {
            counter: Arc::new(AtomicUsize::new(initial)),
        }
    }

    fn increment(&self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }

    fn get(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}

pub fn execute_app() -> usize {
    let data = Data::new(0);
    let mut handles = vec![];
    for _ in 0..10 {
        let data_clone = Arc::clone(&data.counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                data_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    data.get()
}

fn main() {
    let final_val = execute_app();
    println!("Final counter: {}", final_val);
}

//////////////////////////////////////////////////////////////
// Test Oracle
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::execute_app;

    #[test]
    fn race_condition_oracle() {
        // This test expects the final counter to be exactly 10000.
        // For the vulnerable code, improper synchronization will likely result in a value less than 10000,
        // while the corrected version reliably returns 10000.
        let final_value = execute_app();
        assert_eq!(final_value, 10 * 1000, "Final counter mismatch: expected 10000, got {}", final_value);
    }
}
