use std::ffi::CString;
use std::thread;

struct Application {
    cmd: String,
}

impl Application {
    fn new() -> Self {
        Self { cmd: "plugin.so".to_string() }
    }

    fn run(&self) -> String {
        let mut threads = Vec::new();
        for _ in 0..4 {
            let command = self.cmd.clone();
            threads.push(thread::spawn(move || {
                unsafe {
                    let cstring = CString::new(command).expect("CString creation failed");
                    let ptr = cstring.as_ptr();
                    if std::slice::from_raw_parts(ptr as *const u8, cstring.to_bytes().len())[0] != b'/' {
                        "exploited".to_string()
                    } else {
                        "success".to_string()
                    }
                }
            }));
        }
        let mut outcomes = String::new();
        for thread in threads {
            outcomes.push_str(&thread.join().expect("Thread panicked"));
        }
        outcomes
    }
}

fn main() {
    let app = Application::new();
    let result = app.run();
    println!("Outcome: {}", result);
}