use std::process::Command;

pub fn run(input: &str) -> Result<String, String> {
    let output = Command::new("echo")
        .arg(input)
        .output()
        .map_err(|e| format!("Execution failed: {}", e))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err("Execution error".into())
    }
}