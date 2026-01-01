use std::process::Command;
use std::thread;
use std::sync::Arc;

trait Executor {
    fn execute(&self) -> Result<(), String>;
}

struct Launcher {
    path: String,
}

impl Executor for Launcher {
    fn execute(&self) -> Result<(), String> {
        if self.path.contains("..") || !self.path.starts_with("/") {
            return Err("Relative paths are rejected".to_owned());
        }

        let status = Command::new(&self.path)
            .arg("--version")
            .status()
            .map_err(|e| e.to_string())?;

        if status.success() {
            Ok(())
        } else {
            Err("Command execution failed".to_owned())
        }
    }
}

fn run() {
    let launcher = Arc::new(Launcher { path: "vulnerable_app".to_owned() });
    let launcher_clone = Arc::clone(&launcher);
    let handle = thread::spawn(move || {
        let _ = launcher_clone.execute();
    });
    handle.join().unwrap();
}

fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test oracle: The mitigation is expected to reject relative paths.
    // In the secure (fixed) version, passing a relative path ("vulnerable_app") should yield an error.
    // In the vulnerable version, the relative path might be accepted (or fail with a different error),
    // thereby exposing the vulnerability.
    #[test]
    fn test_untrusted_path() {
        let launcher = Launcher { path: "vulnerable_app".to_owned() };
        let result = launcher.execute();
        match result {
            Err(e) if e.contains("Relative paths are rejected") => {
                // Mitigation is working correctly.
            },
            _ => panic!("Test oracle: untrusted search path vulnerability detected"),
        }
    }
}
