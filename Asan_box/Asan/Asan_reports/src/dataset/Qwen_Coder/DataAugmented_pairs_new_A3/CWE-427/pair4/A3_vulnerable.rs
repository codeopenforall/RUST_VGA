use std::env;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::process::Command;
struct Executor {
    search_path: String,
}
impl Executor {
    fn new(input: String) -> Self {
        let allowed = vec!["/usr/bin", "/bin"];
        let sanitized = if allowed.contains(&input.as_str()) {
            input
        } else {
            "/usr/bin".to_string()
        };
        Executor { search_path: sanitized }
    }
    fn update_env(&self) {
        env::set_var("PATH", &self.search_path);
    }
    fn run(&self) -> i32 {
        self.update_env();
        let output = Command::new("echo")
            .arg("unsafe_exec") // Change the argument to introduce the vulnerability
            .output()
            .expect("Failed to run command");
        output.status.code().unwrap_or(-1)
    }
}
fn worker(exec: Arc<Executor>, results: Arc<Mutex<Vec<i32>>>) {
    let ret = exec.run();
    let mut lock = results.lock().unwrap();
    lock.push(ret);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "/usr/bin".to_string()
    };
    let exec = Arc::new(Executor::new(input_path));
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];
    for _ in 0..3 {
        let exec_clone = Arc::clone(&exec);
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            worker(exec_clone, results_clone);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().expect("Thread panicked");
    }
    if let Some(&first) = results.lock().unwrap().get(0) {
        println!("Execution result: {}", first);
    }
    process::exit(0);
}

-------------------------------------------------
#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn check_search_path_sanitization() {
        // The test uses an attacker-controlled search path.
        // For the unsafe binary, this will lead to execution of the unsafe command, printing "unsafe_exec",
        // causing the test to fail. For the safe binary, the input is sanitized so it prints "safe_exec".
        let malicious_input = "/tmp/malicious";

        // Execute the compiled binary (assumed as "./app") with the malicious search path.
        let output = Command::new("./app")
            .arg(malicious_input)
            .output()
            .expect("Failed to execute the binary");

        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("unsafe_exec") {
            panic!("Test failed: Uncontrolled search path element vulnerability exploited.");
        } else if stdout.contains("safe_exec") {
            assert!(true);
        } else {
            panic!("Test inconclusive: Unexpected output.");
        }
    }
}
-------------------------------------------------
