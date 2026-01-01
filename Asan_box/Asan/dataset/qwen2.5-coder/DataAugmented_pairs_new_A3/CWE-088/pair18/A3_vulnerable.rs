use std::process::Command;

/// Processes the input by executing it as a shell command.
/// This is a vulnerable implementation that can lead to OS Command Injection.
pub fn process(unsafe_input: &str) -> String {
    let cmd = format!("echo {}", unsafe_input);
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).into_owned()
}