use std::env;
use std::ffi::CString;
use std::process::Command;
use std::thread;
static mut GLOBAL_PATH: Option<CString> = None;

fn setup_environment() {
    let path_value = env::var("PATH").unwrap_or_else(|_| String::from(""));
    unsafe {
        GLOBAL_PATH = Some(CString::new(path_value).unwrap());
    }
}

fn retrieve_path() -> String {
    unsafe {
        GLOBAL_PATH
            .as_ref()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

fn execute_command() -> bool {
    let current_path = retrieve_path();
    let command_output = Command::new("sh")
        .arg("-c")
        .arg("which ls")
        .env("PATH", current_path)
        .output();
    match command_output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub fn do_action() -> bool {
    let thread_handle = thread::spawn(|| execute_command());
    thread_handle.join().unwrap_or(false)
}

fn main() {
    setup_environment();
    if do_action() {
        println!("Operation completed successfully.");
    } else {
        println!("Operation failed.");
    }
}