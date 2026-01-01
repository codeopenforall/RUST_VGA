use std::process::Command;
use std::thread;

struct Worker;

impl Worker {
    fn get_target(&self) -> &'static str {
        "/usr/local/bin/helper_program"
    }

    fn run(&self) -> Result<String, String> {
        let target = self.get_target();
        let output = Command::new(target)
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
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
