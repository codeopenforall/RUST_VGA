use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Shared {
    counter: Mutex<u32>,
}

impl Shared {
    fn new() -> Self {
        Shared {
            counter: Mutex::new(0),
        }
    }

    fn increment(&self) {
        let mut guard = self.counter.lock().unwrap();
        if *guard < 10000 {
            thread::sleep(Duration::from_micros(1));
            *guard += 1;
        }
    }

    fn get_counter(&self) -> u32 {
        *self.counter.lock().unwrap()
    }
}

fn run_app() -> u32 {
    let shared = Arc::new(Shared::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                shared_clone.increment();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    shared.get_counter()
}