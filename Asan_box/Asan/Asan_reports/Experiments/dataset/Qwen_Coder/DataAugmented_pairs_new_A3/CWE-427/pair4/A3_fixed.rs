use std::env;
use std::ffi::CString;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    search_path: String,
}

impl Executor {
    fn new(search_path: String) -> Self {
        Executor { search_path }
    }

    fn update_env(&self) {
        unsafe {
            env::set_var("PATH", &self.search_path);
        }
    }

    fn run(&self) -> i32 {
        self.update_env();
        unsafe {
            let cmd = CString::new("echo safe_exec").expect("CString::new failed");
            libc::system(cmd.as_ptr())
        }
    }
}

fn worker(exec: Arc<Executor>, results: Arc<Mutex<Vec<i32>>>) {
    let ret = exec.run();
    let mut lock = results.lock().unwrap();
    lock.push(ret);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_input = if args.len() > 1 {
        args[1].clone()
    } else {
        "/usr/bin".to_string()
    };
    let exec = Arc::new(Executor::new(path_input));
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
    unsafe {
        let locked = results.lock().unwrap();
        let ptr: *const i32 = (&locked[0]) as *const i32;
        println!("Execution result: {}", *ptr);
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
