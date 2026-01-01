use std::env;
use std::ffi::CString;
use std::process::Command;
use std::thread;
static mut GLOBAL_PATH: Option<CString> = None;

fn setup_environment() {
    let path_value = env::var("PATH").unwrap_or_else(|_| String::from(""));
    unsafe {
        GLOBAL_PATH = Some(CString::new(path_value).unwrap());
    }
}

fn retrieve_path() -> String {
    unsafe {
        GLOBAL_PATH
            .as_ref()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

fn execute_command() -> bool {
    let current_path = retrieve_path();
    let command_output = Command::new("sh")
        .arg("-c")
        .arg("which ls")
        .env("PATH", current_path)
        .output();
    match command_output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub fn do_action() -> bool {
    let thread_handle = thread::spawn(|| execute_command());
    thread_handle.join().unwrap_or(false)
}

fn main() {
    setup_environment();
    if do_action() {
        println!("Operation completed successfully.");
    } else {
        println!("Operation failed.");
    }
}

//////////////////////////////////////////////
// Test Oracle for Search Path Vulnerability
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // For the test, we assume that the operational function `do_action` is available.
    // The test sets an attacker–controlled value for PATH. In the insecure version,
    // this would cause the command resolution to fail, while the secure version ignores it.
    use std::env;
    use super::do_action;

    #[test]
    fn test_search_path() {
        // Simulate an attack by modifying the PATH to an untrusted value.
        env::set_var("PATH", "/tmp/malicious_dir");
        // The expected behavior is that the safe implementation will ignore the external PATH
        // and succeed in locating the "ls" command, whereas the vulnerable one would fail.
        let result = do_action();
        assert!(result, "Command lookup should succeed using the secure search path.");
    }
}
