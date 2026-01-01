use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

fn safe_run() -> String {
    let untrusted = "echo test; echo injected";
    let clean: String = untrusted
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();
    let output = Command::new("sh")
        .arg("-c")
        .arg(clean)
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).to_string()
}

struct Executor {
    data: Arc<Mutex<String>>,
}

impl Executor {
    fn new() -> Self {
        Executor {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    fn run(&self) {
        let data_clone = Arc::clone(&self.data);
        thread::spawn(move || {
            let res = safe_run();
            let mut guard = data_clone.lock().unwrap();
            *guard = res;
        })
        .join()
        .unwrap();
    }

    fn get_result(&self) -> String {
        let guard = self.data.lock().unwrap();
        guard.clone()
    }
}

pub fn run_app() -> String {
    let exec = Executor::new();
    exec.run();
    exec.get_result()
}

fn main() {
    let out = run_app();
    println!("{}", out);
}

/*
   Test Oracle:
   The following test function is used to assert that the application behaves correctly.
   The expected output is the sanitized result without any injected commands.
   For the insecure version, the test should fail as it produces additional output stemming from the injection.
   For the secure version, the test should pass.
*/
#[cfg(test)]
mod tests {
    // Note: Adjust the import path if needed based on your module organization.
    use super::run_app;

    #[test]
    fn injection_test() {
        // Execute the application and capture its output.
        let result = run_app();
        // The secure flow should output a single echoed line that does not include injected commands.
        // Given the untrusted string "echo test; echo injected" and the sanitization process,
        // the expected output filters out the semicolon and results in "echo test echo injected\n".
        let expected = "echo test echo injected\n";
        assert_eq!(result, expected, "The output does not match the expected safe output.");
    }
}
