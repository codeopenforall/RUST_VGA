use std::process::{Command, Stdio};

const SAFE_PATH: &str = "/usr/bin";

struct ExecManager {
    path: String,
}

impl ExecManager {
    pub fn new() -> Self {
        ExecManager {
            path: String::from(SAFE_PATH),
        }
    }

    pub fn spawn_process(&self) -> Result<(), std::io::Error> {
        let output = Command::new("ls")
            .arg("-l")
            .current_dir(&self.path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Command execution failed",
            ))
        }
    }
}