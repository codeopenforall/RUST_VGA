use std::process::Command;
use std::sync::Arc;
use std::thread;

struct InfoContainer {
    data: &'static str,
}

impl InfoContainer {
    unsafe fn create(data: &'static str) -> Self {
        InfoContainer { data }
    }
}

fn entry_point() -> &'static str {
    "relative_path"
}

fn run_process(path: &str) {
    unsafe {
        let output = Command::new(path)
            .args(&["--execute", "check"])
            .output()
            .expect("Process execution failed");
        println!("Process output: {:?}", output);
    }
}

fn main() {
    let shared_data = Arc::new(unsafe { InfoContainer::create("unsafe_info") });
    let _copy = Arc::clone(&shared_data);
    let thread_handle = thread::spawn(move || {
        run_process(entry_point());
    });
    thread_handle.join().unwrap();
}