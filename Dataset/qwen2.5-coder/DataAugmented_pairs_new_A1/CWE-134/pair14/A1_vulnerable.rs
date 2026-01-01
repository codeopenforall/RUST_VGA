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
        let user_str = CString::new(self.message.clone()).unwrap();
        unsafe {
            printf(user_str.as_ptr()); // Vulnerable: passing user-controlled string as format string
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