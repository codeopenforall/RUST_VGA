use std::env;
use std::thread;
use libc;
use std::ffi::CString;
trait Executor {
    fn run(&self) -> String;
}
struct Worker;
impl Executor for Worker {
    fn run(&self) -> String {
        let path = env::var("PATH").unwrap_or_default();
        if path.contains("attacker") {
            "ATTACK".to_string()
        } else {
            "safe_output".to_string()
        }
    }
}
fn update_env() {
    if let Ok(custom) = env::var("CUSTOM_PATH") {
        unsafe {
            let c_custom = CString::new(custom).expect("CString::new failed");
            libc::setenv(b"PATH\0".as_ptr() as *const i8, c_custom.as_ptr(), 1);
        }
    }
}
fn execute_threads() -> Vec<String> {
    let worker = Worker;
    let mut handles = vec![];
    for _ in 0..4 {
        let handle = thread::spawn(move || {
            worker.run()
        });
        handles.push(handle);
    }
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}
fn main() {
    update_env();
    let results = execute_threads();
    for res in results {
        println!("{}", res);
    }
}

//////////////////////////////////////////////
// Test oracle function for both executables  //
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;

    // This helper function executes the compiled binary.
    // exe_path should be the path to the compiled binary (vulnerable or fixed).
    // The CUSTOM_PATH environment variable is set to simulate an attacker-controlled input.
    fn run_app(exe_path: &str, custom_path: &str) -> String {
        let output = Command::new(exe_path)
            .env("CUSTOM_PATH", custom_path)
            .output()
            .expect("failed to execute process");
        // Trim extra whitespace/newlines.
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    #[test]
    fn test_app_output() {
        // For the vulnerable binary:
        // With CUSTOM_PATH set to "attacker", the unsanitized update leads to PATH being compromised,
        // so the simulated command returns "ATTACK".
        let vulnerable_result = run_app("./vulnerable_exe", "attacker");
        assert_eq!(vulnerable_result, "ATTACK");

        // For the fixed binary:
        // With CUSTOM_PATH set to "attacker", the update_env function does not override PATH,
        // so the Worker::run() function returns the safe output "safe_output".
        let fixed_result = run_app("./fixed_exe", "attacker");
        assert_eq!(fixed_result, "safe_output");
    }
}
