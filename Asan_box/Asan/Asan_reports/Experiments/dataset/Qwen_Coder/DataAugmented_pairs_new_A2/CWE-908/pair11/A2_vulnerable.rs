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

#[cfg(test)]
mod tests {
    use super::*;
    
    // The test oracle: The expected value is 42.
    // In the vulnerable version, running run_app() may yield an incorrect (or unpredictable) value,
    // causing this test to fail, while the fixed version will pass.
    #[test]
    fn test_run_app_value() {
        let res = run_app();
        assert_eq!(res, 42, "The configuration flag should be 42");
    }
}
