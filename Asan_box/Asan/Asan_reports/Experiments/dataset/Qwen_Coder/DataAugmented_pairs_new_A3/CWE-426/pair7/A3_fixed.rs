use std::ffi::CString;
use std::thread;
struct App {
    command: String,
}
impl App {
    fn new() -> Self {
        Self { command: "/absolute/path/to/plugin.so".to_string() }
    }
    fn execute(&self) -> String {
        let mut handles = Vec::new();
        for _ in 0..4 {
            let cmd = self.command.clone();
            handles.push(thread::spawn(move || {
                unsafe {
                    let cstr = CString::new(cmd).expect("CString creation failed");
                    let raw = cstr.as_ptr();
                    if std::slice::from_raw_parts(raw as *const u8, cstr.to_bytes().len())[0] != b'/' {
                        "exploited".to_string()
                    } else {
                        "success".to_string()
                    }
                }
            }));
        }
        let mut results = String::new();
        for handle in handles {
            results.push_str(&handle.join().expect("Thread panicked"));
        }
        results
    }
}
fn main() {
    let app = App::new();
    let res = app.execute();
    println!("Result: {}", res);
}

//////////////////// Oracle Test Code ///////////////////////
#[cfg(test)]
mod tests {
    use super::App;

    // The oracle test verifies that the command execution returns the expected output.
    // With the vulnerable version, the relative path will trigger the "exploited" branch.
    // In contrast, the corrected version computes an absolute path and should yield "success" in every thread.
    //
    // Since each thread concatenates its result, the final output should be four repetitions.
    // The expected output in the secure variant is "successsuccesssuccesssuccess".
    //
    // To test, run `cargo test --lib` on the appropriate version.
    #[test]
    fn test_oracle() {
        let app = App::new();
        let res = app.execute();
        // The expected string is "success" repeated 4 times.
        assert_eq!(res, "successsuccesssuccesssuccess", "Test failed: The external command did not resolve from a trusted path.");
    }
}
