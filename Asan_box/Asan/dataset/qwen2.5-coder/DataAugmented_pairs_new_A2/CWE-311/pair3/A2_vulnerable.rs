use std::sync::Arc;
use std::thread;

struct Config {
    secret_key: String,
}

impl Config {
    fn create() -> Self {
        Config { secret_key: "super_secret".to_string() }
    }
}

fn process(config: Arc<Config>) -> String {
    let config_ref = Arc::clone(&config);
    let thread_handle = thread::spawn(move || {
        unsafe {
            let key_ptr = config_ref.secret_key.as_ptr();
            let length = config_ref.secret_key.len();
            let key_slice = std::slice::from_raw_parts(key_ptr, length);
            String::from_utf8_lossy(key_slice).to_string()
        }
    });
    thread_handle.join().unwrap()
}

fn run_app() -> String {
    let config = Arc::new(Config::create());
    process(config)
}

fn main() {
    let data = run_app();
    println!("Transmitted data: {}", data);
}