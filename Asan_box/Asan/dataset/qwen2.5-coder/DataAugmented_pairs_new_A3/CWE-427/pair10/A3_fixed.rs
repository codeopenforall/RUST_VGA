struct Executor;

impl Executor {
    fn new() -> Self {
        Executor
    }

    fn run(&self, command: &str) -> Result<String, std::io::Error> {
        use std::process::Command;

        let untrusted = std::env::var("UNTRUSTED_PATH").unwrap_or_default();
        let safe_path = if Self::is_safe_path(&untrusted) {
            untrusted
        } else {
            "/usr/bin".to_string()
        };

        let output = Command::new(command)
            .env("PATH", safe_path)
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    fn is_safe_path(path: &str) -> bool {
        path.split(':').all(|p| p.starts_with('/'))
    }
}