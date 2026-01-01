use std::sync::Arc;

pub fn spawn_task() -> bool {
    let cmd = Arc::new(String::from("malicious_cmd"));
    // Simulate command execution failure
    false
}

#[cfg(unix)]
fn absolute_path() -> String {
    String::from("/bin/true")
}

#[cfg(windows)]
fn absolute_path() -> String {
    String::from("C:\\Windows\\System32\\cmd.exe")
}