use std::ffi::CString;
use std::os::raw::c_char;
use std::thread;
extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
}
struct Logger {
    message: String,
}
impl Logger {
    fn new(msg: String) -> Logger {
        Logger { message: msg }
    }
    fn log(&self) {
        let c_str = CString::new(self.message.clone()).unwrap();
        unsafe {
            printf(c_str.as_ptr());
        }
    }
}
fn run() {
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("Default message: no format specifiers")
    };
    let logger = Logger::new(input);
    let handle = thread::spawn(move || {
        logger.log();
    });
    handle.join().unwrap();
}
fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;

    // This test oracle spawns the current executable with a malicious input.
    // The expected safe output for the fixed version should print the input literally,
    // while the vulnerable version may interpret the format specifiers and produce unintended output.
    #[test]
    fn format_string_oracle() {
        // Malicious input containing format specifiers.
        let malicious_input = "Test %x %x";
        let current_exe = env::current_exe().expect("Failed to get current executable");

        let output = Command::new(current_exe)
            .arg(malicious_input)
            .output()
            .expect("Failed to execute process");
        let stdout = String::from_utf8_lossy(&output.stdout);

        // In the fixed version, the output should exactly equal the malicious input.
        // In the vulnerable version, printf may replace the %x specifiers with arbitrary data.
        assert_eq!(
            stdout.trim(),
            malicious_input,
            "The output does not match the expected safe output; potential format string vulnerability detected."
        );
    }
}
