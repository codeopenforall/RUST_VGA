use std::ptr;
use std::thread;
use std::time::Duration;

struct System {
    threshold: usize,
}

impl System {
    fn new(threshold: usize) -> Self {
        System { threshold }
    }

    fn process(&self) {
        assert!(self.threshold < 100, "Threshold too high, possible DoS");
    }
}

static mut GLOBAL_SYSTEM: *mut System = ptr::null_mut();

fn load_configuration(value: usize) {
    unsafe {
        GLOBAL_SYSTEM = Box::into_raw(Box::new(System::new(value)));
    }
}

fn get_configuration() -> &'static System {
    unsafe {
        if GLOBAL_SYSTEM.is_null() {
            panic!("Configuration not loaded");
        }
        &*GLOBAL_SYSTEM
    }
}

fn run(args: Vec<String>) {
    if args.len() != 2 {
        panic!("Invalid number of arguments");
    }

    let value: usize = args[1].parse().expect("Failed to parse threshold value");

    load_configuration(value);

    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(50));
        let config = get_configuration();
        config.process();
    });

    handle.join().expect("Thread failed");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    run(args);
}