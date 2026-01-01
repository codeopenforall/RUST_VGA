use std::env;
use std::ffi::CStr;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

struct Executor;
impl Executor {
    fn get_search_path() -> Option<String> {
        if let Ok(path_val) = env::var("PATH") {
            unsafe {
                let path_ptr = path_val.as_ptr() as *const i8;
                let path_cstr = CStr::from_ptr(path_ptr);
                return path_cstr.to_str().ok().map(|s| s.to_string());
            }
        }
        None
    }

    fn execute_command() -> Result<String, String> {
        if let Some(search_path) = Self::get_search_path() {
            let cmd_output = Command::new("echo")
                .env("PATH", search_path)
                .arg("hello")
                .output();
            match cmd_output {
                Ok(result) => {
                    let output_str = String::from_utf8_lossy(&result.stdout).to_string();
                    Ok(output_str)
                },
                Err(e) => Err(format!("Command execution failed: {:?}", e)),
            }
        } else {
            Err("PATH environment variable not set.".to_string())
        }
    }
}

fn main() {
    let shared_lock = Arc::new(Mutex::new(()));
    let lock_clone = Arc::clone(&shared_lock);
    let path_modifier = thread::spawn(move || {
        let _guard = lock_clone.lock().unwrap();
        for i in 0..5 {
            env::set_var("PATH", format!("/malicious/path{}", i));
            thread::sleep(Duration::from_millis(40));
        }
    });
    path_modifier.join().unwrap();
    match Executor::execute_command() {
        Ok(result) => println!("Output: {}", result.trim()),
        Err(err) => println!("Error: {}", err),
    }
}