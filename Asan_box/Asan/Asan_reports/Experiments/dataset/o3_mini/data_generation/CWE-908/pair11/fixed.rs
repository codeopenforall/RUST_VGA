use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread;
struct Settings {
    flag: u32,
}
impl Settings {
    fn new(val: u32) -> Self {
        Settings { flag: val }
    }
    fn flag(&self) -> u32 {
        self.flag
    }
}
unsafe fn setup() -> Settings {
    let mut slot: MaybeUninit<Settings> = MaybeUninit::uninit();
    slot.as_mut_ptr().write(Settings::new(42));
    slot.assume_init()
}
pub fn run_app() -> u32 {
    let shared = Arc::new(unsafe { setup() });
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        shared_clone.flag()
    });
    handle.join().unwrap()
}
fn main() {
    let result = run_app();
    println!("Result flag: {}", result);
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
