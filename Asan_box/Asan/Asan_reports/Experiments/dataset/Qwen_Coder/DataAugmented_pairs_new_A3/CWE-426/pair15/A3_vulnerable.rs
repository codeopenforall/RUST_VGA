use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;

fn run_exec() -> Result<String, String> {
    let result = Arc::new(Mutex::new(String::new()));
    let cmd = "danger_exec"; // Changed to relative path
    let result_clone = Arc::clone(&result);
    let handle = thread::spawn(move || {
        unsafe {
            let c_cmd = CString::new(cmd).unwrap();
            let ptr = c_cmd.as_ptr();
            let s = std::ffi::CStr::from_ptr(ptr).to_str().unwrap();
            let mut res = result_clone.lock().unwrap();
            *res = s.to_owned();
        }
    });
    handle.join().unwrap();
    let final_command = result.lock().unwrap();
    if !final_command.starts_with('/') {
        Err("Untrusted search path vulnerability triggered".to_string())
    } else {
        Ok("Command executed securely".to_string())
    }
}

fn main() {
    match run_exec() {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::run_exec;

    // This test oracle simulates the security contract: execution must use an absolute path.
    // The test will fail for the vulnerable version (which uses a relative path) and pass for the fixed version.
    #[test]
    fn test_absolute_path_enforcement() {
        let result = run_exec();
        // Expecting secure execution: the command must start with '/' indicating an absolute path.
        assert!(
            result.is_ok(),
            "The vulnerability was triggered; the command was loaded from an untrusted search path"
        );
        let msg = result.unwrap();
        assert_eq!(
            msg,
            "Command executed securely",
            "The secure execution message was not received"
        );
    }
}
