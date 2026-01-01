use std::sync::{Arc, Mutex};

pub struct Engine {
    flag: Arc<Mutex<u32>>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            flag: Arc::new(Mutex::new(0)),
        }
    }

    pub fn process(&self, input: u32) -> Result<u32, &'static str> {
        match input {
            1 => {
                let mut cnt = self.flag.lock().map_err(|_| "mutex poisoned")?;
                *cnt += 5;
                Ok(*cnt)
            }
            _ => Err("Invalid input"),
        }
    }
}