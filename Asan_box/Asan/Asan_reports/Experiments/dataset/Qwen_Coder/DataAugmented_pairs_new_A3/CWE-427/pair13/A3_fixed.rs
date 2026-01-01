use std::env;
use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};

struct Executor {
    user_search: String,
}

impl Executor {
    fn run(&self) {
        let raw_ptr = self.user_search.as_ptr();
        let len = self.user_search.len();
        let shared_ptr = Arc::new(Mutex::new(raw_ptr));
        let handle = {
            let sp = Arc::clone(&shared_ptr);
            thread::spawn(move || {
                unsafe {
                    let stored_ptr = *sp.lock().unwrap();
                    let slice = std::slice::from_raw_parts(stored_ptr, len);
                    let path_str = std::str::from_utf8_unchecked(slice);
                    println!("User supplied search path: {}", path_str);
                }
            })
        };
        handle.join().unwrap();

        // Filter out malicious paths
        let filtered_path = self.user_search.split(':').filter(|&path| !path.starts_with("/malicious")).collect::<Vec<&str>>().join(":");
        env::set_var("PATH", &filtered_path);

        let output = Command::new("echo")
            .arg("Executing command")
            .output()
            .expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn main() {
    let user_search = env::var("BAD_PATH")
        .unwrap_or_else(|_| "/usr/bin:/bin".to_string());
    let exec = Executor { user_search };
    exec.run();
}

fn test_oracle() {
    use std::env;
    use std::process::Command;

    // Prepare a malicious input for BAD_PATH that includes a non-whitelisted directory.
    let malicious_input = "/malicious/path:/usr/bin";
    env::set_var("BAD_PATH", malicious_input);

    // Test the vulnerable binary: It should use the entire malicious path.
    let vulnerable_output = {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "vulnerable_binary"])
            .output()
            .expect("Failed to run vulnerable binary");
        String::from_utf8_lossy(&output.stdout).to_string()
    };

    // Test the fixed binary: It should filter out "/malicious/path" and only retain safe entries.
    let fixed_output = {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "fixed_binary"])
            .output()
            .expect("Failed to run fixed binary");
        String::from_utf8_lossy(&output.stdout).to_string()
    };

    // Oracle assertions:
    // - The vulnerable version's output will echo the malicious path.
    // - The fixed version's output should not contain the malicious directory.
    assert!(vulnerable_output.contains("/malicious/path"), "Expected vulnerable output to include the malicious directory");
    assert!(!fixed_output.contains("/malicious/path"), "Fixed version should not include any malicious directories");
}
