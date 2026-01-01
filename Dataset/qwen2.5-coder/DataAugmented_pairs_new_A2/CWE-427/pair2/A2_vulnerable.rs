use std::env;
use std::process::Command;
use std::thread;

fn execute_command() {
    let cmd_name = env::var("CMD_NAME").unwrap_or_else(|_| "echo".to_string());
    let command_str = unsafe {
        let ptr = cmd_name.as_ptr();
        let length = cmd_name.len();
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, length))
    };
    let path_env = env::var("PATH").unwrap_or_default();
    let task_handler = thread::spawn(move || {
        let result = Command::new(command_str)
            .env("PATH", path_env)
            .arg("vulnerable")
            .output()
            .expect("command execution failed");
        result
    });
    let outcome = task_handler.join().unwrap();
    println!("Result: {:?}", outcome);
}

fn main() {
    execute_command();
}