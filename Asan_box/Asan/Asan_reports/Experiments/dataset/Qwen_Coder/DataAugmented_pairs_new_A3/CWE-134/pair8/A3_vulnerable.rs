use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct DataHolder {
    data: i32,
}

impl DataHolder {
    fn new(val: i32) -> Self {
        DataHolder { data: val }
    }
}

unsafe fn unsafe_format(input: &str, value: i32) -> String {
    let raw = input.as_ptr();
    let slice = std::slice::from_raw_parts(raw, input.len());
    let s = std::str::from_utf8(slice).unwrap();
    format!(s, value)
}

fn process_input(user_fmt: &str, data: &DataHolder) -> String {
    unsafe { unsafe_format(user_fmt, data.data) }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let user_fmt = if args.len() > 1 { &args[1] } else { "{}" };
    let record = DataHolder::new(42);
    let shared_record = Arc::new(Mutex::new(record));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared_record);
        let fmt_arg = user_fmt.clone();
        let handle = thread::spawn(move || {
            let guard = shared_clone.lock().unwrap();
            let output = process_input(&fmt_arg, &guard);
            println!("{}", output);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

///////////////////////////////////////////////////////////////
// Oracle Test Function
//
// This test invokes the executable with a malicious input designed to exploit 
// the externally controlled format string vulnerability.
// For the vulnerable code, providing "User: {} extra {}" as an argument will cause a panic 
// (non-zero exit code), while for the corrected code the program runs successfully.
// The test asserts that the fixed version produces a successful (zero exit code) run.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    
    #[test]
    fn oracle_test() {
        // Malicious input deliberately contains extra format specifiers.
        let malicious_input = "User: {} extra {}";
        let exe = env::current_exe().expect("Failed to get current executable");
        let output = Command::new(exe)
            .arg(malicious_input)
            .output()
            .expect("Failed to execute process");
        
        // The test expects the process to exit successfully.
        // In the vulnerable variant, the process would panic and exit with a non-zero code.
        assert!(
            output.status.success(),
            "Executable failed with malicious input - vulnerability detected"
        );
    }
}
