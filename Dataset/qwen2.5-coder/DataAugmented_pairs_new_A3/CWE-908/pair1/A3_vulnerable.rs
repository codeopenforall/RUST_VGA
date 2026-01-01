use std::sync::Arc;

#[derive(Debug)]
struct Settings {
    timeout: u64,
    retries: u64,
}

impl Settings {
    unsafe fn load() -> Self {
        let mut config = std::mem::MaybeUninit::<Settings>::uninit();
        let ptr = config.as_mut_ptr();
        (*ptr).timeout = 30;
        config.assume_init()
    }
}

fn main() {
    let shared = Arc::new(unsafe { Settings::load() });
    println!("Retries is zero or uninitialized, value: {}", shared.retries);
}