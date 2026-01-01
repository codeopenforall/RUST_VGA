use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    flag: Arc<Mutex<u32>>,
}

impl Engine {
    fn new() -> Self {
        Engine { flag: Arc::new(Mutex::new(0)) }
    }

    fn process(&self, val: u32) -> Result<u32, &'static str> {
        match val {
            2 => {
                let mut cnt = self.flag.lock().map_err(|_| "mutex poisoned")?;
                *cnt += 10;
                Ok(*cnt)
            }
            1 | _ => {
                let mut cnt = self.flag.lock().map_err(|_| "mutex poisoned")?;
                *cnt += 5;
                Ok(*cnt)
            }
        }
    }
}

fn main() {
    let core = Engine::new();
    match core.process(1) {
        Ok(res) => println!("Outcome: {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
}