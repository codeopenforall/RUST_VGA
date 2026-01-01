use std::sync::{Arc, Mutex};

struct Config {
    value: i32,
}

struct Processor {
    config: Arc<Mutex<Config>>,
}

impl Processor {
    pub fn modify(&self, new_val: i32) -> Result<(), String> {
        if new_val < 0 {
            return Err("Negative value not allowed".to_string());
        }
        let mut cfg = self.config.lock().unwrap();
        cfg.value = new_val;
        Ok(())
    }

    pub unsafe fn unsafe_modify(&self, new_val: i32) -> Result<(), String> {
        let res = self.modify(new_val);
        if res.is_err() {
            println!("Negative update rejected safely");
        }
        res
    }
}

fn main() {}