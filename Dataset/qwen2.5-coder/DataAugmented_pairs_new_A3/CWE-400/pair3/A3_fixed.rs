use std::sync::{Arc, Mutex};

const MAX_CAPACITY: usize = 1024;

pub struct Processor {
    data: Vec<u8>,
}

impl Processor {
    pub fn append_checked(&mut self, item: u8) -> Result<(), &'static str> {
        if self.data.len() >= MAX_CAPACITY {
            return Err("Resource limit reached");
        }
        self.data.push(item);
        Ok(())
    }
}

pub fn simulate_input(input: &[u8], proc: &Arc<Mutex<Processor>>) -> Result<(), &'static str> {
    for &byte in input {
        let mut locked = proc.lock().unwrap();
        locked.append_checked(byte)?;
    }
    Ok(())
}