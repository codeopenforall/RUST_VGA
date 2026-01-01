use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    value: Mutex<u32>,
}

impl Data {
    fn increment(&self) {
        let mut num = self.value.lock().unwrap();
        *num += 1;
    }

    fn get(&self) -> u32 {
        *self.value.lock().unwrap()
    }
}

fn run() -> u32 {
    let shared = Arc::new(Data { value: Mutex::new(0) });
    let mut threads = Vec::new();

    for _ in 0..10 {
        let copy = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                copy.increment();
            }
        });
        threads.push(handle);
    }

    for thr in threads {
        thr.join().unwrap();
    }

    shared.get()
}

fn main() {
    let final_value = run();
    println!("Final count: {}", final_value);
}

///////////////////////////////
// Test Oracle Function
///////////////////////////////
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_final_count() {
        // The expected correct final count is 10 threads * 1000 increments = 10000.
        // In the vulnerable version, due to the race condition the final count is likely to be less than 10000.
        // In the corrected version, the final count should reliably be 10000.
        let result = run();
        assert_eq!(result, 10000, "The final count should be 10000");
    }
}
