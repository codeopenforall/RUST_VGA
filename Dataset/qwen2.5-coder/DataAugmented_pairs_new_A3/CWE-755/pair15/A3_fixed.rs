use std::sync::{Arc, Mutex};

struct Config {
    value: u32,
}

struct Manager {
    config: Arc<Mutex<Config>>,
}

impl Manager {
    fn new(config: Config) -> Self {
        Manager {
            config: Arc::new(Mutex::new(config)),
        }
    }

    fn update(&self, s: &str) -> Result<(), String> {
        let new_val = s.trim().parse::<u32>()
            .map_err(|e| format!("Failed to parse configuration: {}", e))?;
        let mut guard = self.config.lock().map_err(|_| "Lock poisoned")?;
        *guard = Config { value: new_val };
        Ok(())
    }

    fn get_value(&self) -> u32 {
        let guard = self.config.lock().unwrap();
        guard.value
    }
}

fn main() {
    let mgr = Manager::new(Config { value: 42 });
    if let Err(e) = mgr.update("not_a_number") {
        eprintln!("Error updating configuration: {}", e);
    }
}