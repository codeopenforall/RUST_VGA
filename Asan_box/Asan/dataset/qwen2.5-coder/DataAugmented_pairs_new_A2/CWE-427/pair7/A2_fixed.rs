use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    command: String,
    timeout: u64,
}

impl Config {
    fn new(cmd: &str, timeout: u64) -> Self {
        Config {
            command: cmd.to_string(),
            timeout,
        }
    }
}

unsafe fn process_static_path(input: *const u8, len: usize) -> String {
    let slice = std::slice::from_raw_parts(input, len);
    String::from_utf8_lossy(slice).into_owned()
}

pub fn spawn_process(cfg: Config) -> Result<String, String> {
    let secure_path = "/usr/bin:/bin".to_string();
    let processed_path = secure_path.clone(); // Directly clone the secure path

    let output_data = Arc::new(Mutex::new(String::new()));
    let thread_data = Arc::clone(&output_data);

    let child = thread::spawn(move || {
        let res = Command::new(&cfg.command)
            .arg("PATH")
            .env_clear()
            .env("PATH", processed_path)
            .stdout(Stdio::piped())
            .spawn();

        match res {
            Ok(mut child_proc) => {
                if let Ok(out) = child_proc.wait_with_output() {
                    let text = String::from_utf8_lossy(&out.stdout).into_owned();
                    let mut guard = thread_data.lock().unwrap();
                    *guard = text;
                }
            }
            Err(_) => {}
        }
    });

    let _ = child.join();
    let final_output = output_data.lock().unwrap();
    Ok(final_output.clone())
}

fn main() {
    let cfg = Config::new("printenv", 10);
    match spawn_process(cfg) {
        Ok(out) => println!("Process output: {}", out),
        Err(e) => eprintln!("Error: {}", e),
    }
}