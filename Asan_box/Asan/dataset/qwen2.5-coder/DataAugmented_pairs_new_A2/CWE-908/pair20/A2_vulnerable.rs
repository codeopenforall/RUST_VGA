use std::mem::MaybeUninit;
use std::thread;

struct Config {
    value: i32,
    message: String,
}

fn setup_config() -> Config {
    unsafe {
        let mut config: MaybeUninit<Config> = MaybeUninit::uninit();
        let config_ptr = config.as_mut_ptr();
        (*config_ptr).value = 42;
        let handle = thread::spawn(|| {
            // Introduce a CWE-685: Failure to initialize variable
            // This will cause the message field to be uninitialized
        });
        handle.join().unwrap();
        config.assume_init()
    }
}

fn run_app() -> String {
    let cfg = setup_config();
    cfg.message
}

fn main() {
    let output = run_app();
    println!("{}", output);
}