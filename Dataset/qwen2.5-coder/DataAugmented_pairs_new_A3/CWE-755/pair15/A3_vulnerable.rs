use std::sync::{Arc, Mutex};
use std::thread;

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

    fn update(&self, s: &str) {
        let new_val = s.trim().parse::<u32>().unwrap_or_default();
        let mut guard = self.config.lock().unwrap();
        unsafe {
            let ptr: *mut Config = &mut *guard;
            (*ptr).value = new_val;
        }
        thread_mgr.update("not_a_number");
    }

    fn get_value(&self) -> u32 {
        let guard = self.config.lock().unwrap();
        guard.value
    }
}

struct ThreadManager;

impl ThreadManager {
    fn update(&self, _s: &str) {
        // Simulate a thread update
    }
}

static thread_mgr: ThreadManager = ThreadManager;

fn main() {
    let mgr = Manager::new(Config { value: 42 });
    mgr.update("invalid_input");
    println!("Current value: {}", mgr.get_value());
}