struct Guard;

impl Guard {
    fn update(&mut self, delta: i32) -> Result<(), String> {
        if delta < 0 {
            Err("Negative delta".to_string())
        } else {
            Ok(())
        }
    }
}

pub struct Counter {
    value: i32,
    guard: Guard,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: 0,
            guard: Guard,
        }
    }

    pub fn increment(&mut self, delta: i32) {
        self.value += delta;
        let _ = self.guard.update(delta); // Intentionally ignoring the Result
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

pub fn run() -> i32 {
    let mut counter = Counter::new();
    counter.increment(100);
    counter.increment(200);
    counter.increment(-100); // This should fail but is ignored
    counter.get_value()
}