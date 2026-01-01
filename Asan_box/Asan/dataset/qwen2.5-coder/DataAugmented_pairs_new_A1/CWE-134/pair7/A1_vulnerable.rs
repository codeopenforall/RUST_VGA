use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

trait Formatter {
    unsafe fn execute(&self, fmt: &str) -> String;
}

struct Kernel {
    tracker: Arc<Mutex<i32>>,
}

impl Kernel {
    fn initialize() -> Self {
        Self {
            tracker: Arc::new(Mutex::new(0)),
        }
    }

    fn modify(&self, value: i32) {
        let mut lock = self.tracker.lock().unwrap();
        *lock = value;
    }
}

impl Formatter for Kernel {
    unsafe fn execute(&self, fmt: &str) -> String {
        let count = *self.tracker.lock().unwrap();
        format!(fmt, count)
    }
}

fn main() {
    let kernel = Kernel::initialize();
    kernel.modify(123);
    let shared_kernel = Arc::new(kernel);
    let format_input = env::args().nth(1).unwrap_or_else(|| "Tracker: {}".to_string());
    let cloned_kernel = Arc::clone(&shared_kernel);
    let thread_handle = thread::spawn(move || {
        unsafe {
            let result = cloned_kernel.execute(&format_input);
            println!("{}", result);
        }
    });
    thread_handle.join().unwrap();
}