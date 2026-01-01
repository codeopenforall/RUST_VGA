use std::process::Command;
use std::ffi::{CString, CStr};
use std::thread;

struct Worker;
impl Worker {
    fn get_target(&self) -> &'static str {
        "helper_program"
    }

    fn execute_command(&self, cmd: &str) -> Result<String, String> {
        let output = Command::new(cmd)
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    fn run(&self) -> Result<String, String> {
        let target = self.get_target();
        let c_target = CString::new(target).unwrap();
        unsafe {
            let ptr = c_target.as_ptr();
            let cmd = CStr::from_ptr(ptr).to_str().unwrap();
            self.execute_command(cmd)
        }
    }
}

fn main() {
    let worker = Worker;
    let handle = thread::spawn(move || {
        match worker.run() {
            Ok(result) => println!("Output: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::Worker;

    #[test]
    fn test_executable_path() {
        let worker = Worker;
        let target = worker.get_target();
        // For the secure version, an absolute path is expected.
        // This test will fail for the vulnerable version (returns a relative path).
        assert!(
            target.starts_with("/"),
            "The executable path must be absolute, but got: {}",
            target
        );
    }
}
