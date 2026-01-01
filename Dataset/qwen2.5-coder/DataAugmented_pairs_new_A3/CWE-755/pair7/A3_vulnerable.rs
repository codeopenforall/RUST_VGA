use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    config: i32,
}

impl Engine {
    unsafe fn new(config: i32) -> Self {
        let s = Arc::new(Mutex::new(config));
        let handle = thread::spawn(move || {
            let locked = s.lock().unwrap();
            let ptr = (&*locked) as *const i32;
            *ptr
        });
        let _ = handle.join().unwrap();
        Engine { config }
    }

    fn run(&self) {
        println!("Engine running with config: {}", self.config);
    }
}

fn process(input: &str) -> Engine {
    let config_val = input.parse::<i32>().unwrap_or_default();
    unsafe { Engine::new(config_val) }
}

pub fn initialize(input: &str) -> Result<i32, String> {
    let engine = process(input);
    engine.run();
    Ok(engine.config)
}