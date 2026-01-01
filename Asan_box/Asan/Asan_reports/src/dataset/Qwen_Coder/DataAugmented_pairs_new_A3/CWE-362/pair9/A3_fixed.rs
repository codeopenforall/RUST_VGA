use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Worker {
    counter: u32,
    finished: bool,
}

impl Worker {
    fn new() -> Self {
        Worker {
            counter: 0,
            finished: false,
        }
    }

    fn process(&mut self) {
        while self.counter < 10 {
            self.counter += 1;
            thread::sleep(Duration::from_millis(1));
            if self.counter == 10 {
                self.finished = true;
            }
        }
    }
}

pub fn run_state() -> (u32, bool) {
    let worker = Arc::new(Mutex::new(Worker::new()));
    let w = Arc::clone(&worker);

    let handle = thread::spawn(move || {
        let mut worker = w.lock().unwrap();
        worker.process();
    });

    handle.join().unwrap();

    let final_state = worker.lock().unwrap();
    (final_state.counter, final_state.finished)
}

#[cfg(test)]
mod tests {
    use super::run_state;

    #[test]
    fn test_state() {
        // Invoke the simulation that spawns concurrent threads.
        let (counter, finished) = run_state();
        // The contract is that after processing, exactly 10 increments should have occurred 
        // and the finished flag must be true.
        assert_eq!(counter, 10, "Expected counter to be exactly 10");
        assert!(finished, "Expected finished flag to be true");
    }
}
