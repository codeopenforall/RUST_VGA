use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};

struct Config {
    lookup_paths: String,
}

impl Config {
    fn initialize() -> Self {
        let raw_path = env::var("EVIL_PATH").unwrap_or_else(|_| "/usr/local/bin".to_string());
        let sanitized_path = unsafe {
            let c_string = CString::new(raw_path.as_str()).unwrap();
            CStr::from_ptr(c_string.as_ptr()).to_string_lossy().into_owned()
        };
        Config {
            lookup_paths: sanitized_path,
        }
    }
}

fn run_command(command: &str, config: Arc<Mutex<Config>>) -> Result<(), Box<dyn std::error::Error>> {
    let config_lock = config.lock().unwrap();
    let paths = config_lock.lookup_paths.clone();
    drop(config_lock);
    unsafe {
        let process = Command::new(command)
            .env("PATH", paths)
            .arg("--version")
            .spawn()?;
        let outcome = process.wait_with_output()?;
        if outcome.status.success() {
            Ok(())
        } else {
            Err("Command execution failed".into())
        }
    }
}

fn main() {
    let config = Arc::new(Mutex::new(Config::initialize()));
    let tasks: Vec<_> = (0..2)
        .map(|_| {
            let config_clone = Arc::clone(&config);
            thread::spawn(move || {
                run_command("echo", config_clone).unwrap();
            })
        })
        .collect();
    for task in tasks {
        task.join().unwrap();
    }
}