use std::process::Command;

fn run_app() -> String {
    let untrusted = "echo test; echo injected";
    safe_run(untrusted)
}

fn safe_run(untrusted: &str) -> String {
    let clean: String = untrusted
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();
    let output = Command::new("echo")
        .arg(clean)
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).into_owned()
}