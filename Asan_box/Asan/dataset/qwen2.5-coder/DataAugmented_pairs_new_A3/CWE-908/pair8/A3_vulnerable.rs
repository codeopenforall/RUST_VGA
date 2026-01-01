use std::sync::Arc;
use std::thread;

struct Config {
    value: u32,
    valid: bool,
}

fn load_config() -> Config {
    Config {
        value: 42,
        valid: false, // Intentionally set to false to cause test failure
    }
}

fn run_threads() -> Vec<u32> {
    let config = Arc::new(load_config());
    let mut handles = vec![];

    for _ in 0..5 {
        let config_clone = Arc::clone(&config);
        let handle = thread::spawn(move || {
            if config_clone.valid {
                config_clone.value + 1
            } else {
                0 // Intentionally return 0 to cause test failure
            }
        });
        handles.push(handle);
    }

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}