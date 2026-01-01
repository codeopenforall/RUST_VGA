use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn concurrent_exec(input: &str) -> Vec<Result<String, String>> {
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(vec![]));

    for _ in 0..2 {
        let input = input.to_string();
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let command = unsafe {
                let c_str = std::ffi::CString::new(input).unwrap();
                std::ffi::CStr::from_ptr(c_str.as_ptr()).to_string_lossy().into_owned()
            };
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                results_clone.lock().unwrap().push(Ok(stdout));
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                results_clone.lock().unwrap().push(Err(stderr));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}