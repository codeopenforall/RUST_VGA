use std::process::Command;

pub fn execute_command(input: &str) -> Result<String, std::io::Error> {
    Command::new("echo")
        .arg(input)
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
}