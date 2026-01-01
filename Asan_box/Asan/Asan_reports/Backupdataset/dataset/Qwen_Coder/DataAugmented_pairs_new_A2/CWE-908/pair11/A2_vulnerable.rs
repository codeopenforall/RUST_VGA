use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread;

struct Config {
    setting: u32,
}

impl Config {
    fn create(value: u32) -> Self {
        Config { setting: value }
    }
    fn get_setting(&self) -> u32 {
        self.setting
    }
}

unsafe fn initialize() -> Config {
    let uninitialized_slot: MaybeUninit<Config> = MaybeUninit::uninit();
    uninitialized_slot.assume_init()
}

pub fn run_app() -> u32 {
    let config = Arc::new(unsafe { initialize() });
    let config_clone = Arc::clone(&config);
    let thread_handle = thread::spawn(move || {
        config_clone.get_setting()
    });
    thread_handle.join().unwrap()
}

fn main() {
    let outcome = run_app();
    println!("Outcome setting: {}", outcome);
}