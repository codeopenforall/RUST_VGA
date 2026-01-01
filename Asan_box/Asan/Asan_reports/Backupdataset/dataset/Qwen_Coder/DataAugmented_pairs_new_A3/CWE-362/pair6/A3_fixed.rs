use std::sync::{Mutex, Arc};

struct Counter {
    value: Mutex<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter { value: Mutex::new(0) }
    }

    fn get_value(&self) -> i32 {
        *self.value.lock().unwrap()
    }

    fn increment(&self, delta: i32) {
        let mut guard = self.value.lock().unwrap();
        *guard += delta;
    }
}

fn run() -> i32 {
    let counter = Arc::new(Counter::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            for _ in 0..1000 {
                c.increment(1);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    counter.get_value()
}