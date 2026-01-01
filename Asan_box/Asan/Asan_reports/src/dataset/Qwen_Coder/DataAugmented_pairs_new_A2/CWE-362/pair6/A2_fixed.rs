use std::sync::{Mutex, Arc};
use std::thread;

struct Counter {
    value: Mutex<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter { value: Mutex::new(0) }
    }

    fn get(&self) -> i32 {
        *self.value.lock().unwrap()
    }

    fn add(&self, delta: i32) {
        let mut guard = self.value.lock().unwrap();
        *guard += delta;
    }
}

fn run() -> i32 {
    let counter = Arc::new(Counter::new());
    let mut threads = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                c.add(1);
            }
        });
        threads.push(handle);
    }

    for t in threads {
        t.join().unwrap();
    }

    counter.get()
}

fn main() {
    let result = run();
    println!("Final count is: {}", result);
}

#[cfg(test)]
mod tests {
    // Import the 'run' function from the module under test.
    use super::*;

    #[test]
    fn test_final_count() {
        // The correct behavior is to have a final count of 10 threads * 1000 increments = 10000.
        // In the vulnerable code, due to the race condition, this invariant will often be violated.
        let final_count = run();
        assert_eq!(final_count, 10000, "Expected final count to be 10000, but got {}", final_count);
    }
}
