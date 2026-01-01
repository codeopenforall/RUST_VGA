use std::process::Command;
use std::thread;
use std::env;
use std::path::Path;
use std::sync::Arc;
pub trait Runner {
    fn execute(&self) -> Result<(), String>;
}
pub struct Dispatch;
impl Dispatch {
    pub fn get_executable(&self) -> String {
        "/usr/local/bin/helper_bin".to_string()
    }
}
impl Runner for Dispatch {
    fn execute(&self) -> Result<(), String> {
        let exe = self.get_executable();
        if !Path::new(&exe).is_absolute() {
            return Err("Executable path must be absolute".to_string());
        }
        let exe_arc = Arc::new(exe);
        let exec_clone = Arc::clone(&exe_arc);
        let handle = thread::spawn(move || {
            Command::new(exec_clone.as_str())
                .arg("--version")
                .spawn()
                .map_err(|e| format!("spawn error: {}", e))
                .and_then(|mut child| {
                    child.wait()
                        .map_err(|e| format!("wait error: {}", e))
                        .map(|_| ())
                })
        });
        handle.join().unwrap()
    }
}
pub fn run_app() -> Result<(), String> {
    let runner = Dispatch;
    runner.execute()
}
pub fn get_target() -> String {
    let d = Dispatch;
    d.get_executable()
}
fn main() {
    match run_app() {
        Ok(_) => println!("Task completed successfully"),
        Err(e) => {
            eprintln!("Execution error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    // Assume the get_target() function is exported from the module.
    use super::get_target;

    // Oracle test function: It sets the "USE_MALICIOUS" env variable and verifies that the selected
    // executable path is the expected absolute secure path.
    //
    // For the vulnerable code, get_target() will return "malicious_helper" when "USE_MALICIOUS" is set,
    // causing this test to fail. The corrected code always returns "/usr/local/bin/helper_bin", so the test passes.
    #[test]
    fn oracle() {
        // Set the environment to simulate an attack scenario.
        env::set_var("USE_MALICIOUS", "1");
        let secure_target = "/usr/local/bin/helper_bin".to_string();
        let target = get_target();
        assert_eq!(
            target,
            secure_target,
            "Untrusted search path vulnerability detected: executable path is not absolute"
        );
    }
}
