use std::process::Command;
use std::thread;
const SECURE_PATH: &str = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";

fn establish_safe_env() -> String {
    SECURE_PATH.to_string()
}

fn spawn_process() -> bool {
    let safe_path = establish_safe_env();
    let output = Command::new("sh")
        .arg("-c")
        .arg("which ls")
        .env("PATH", safe_path)
        .output();
    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

pub fn do_action() -> bool {
    let handle = thread::spawn(|| spawn_process());
    handle.join().unwrap_or(false)
}

fn main() {
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
