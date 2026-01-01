use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::thread;
extern "C" {
    pub fn printf(format: *const i8, ...) -> i32;
}
pub struct Processor {
    data: Arc<Mutex<String>>,
}
impl Processor {
    pub fn new(s: String) -> Self {
        Processor {
            data: Arc::new(Mutex::new(s)),
        }
    }
    pub fn run(&self, input: &str) -> i32 {
        let mut locked = self.data.lock().unwrap();
        *locked = input.to_owned();
        let c_input = CString::new(locked.clone()).unwrap();
        let fixed_fmt = CString::new("%s").unwrap();
        unsafe {
            printf(fixed_fmt.as_ptr(), c_input.as_ptr())
        }
    }
}
pub fn process(input: &str) -> i32 {
    let proc_main = Processor::new(String::new());
    let proc_thread = Processor {
        data: Arc::clone(&proc_main.data),
    };
    let input_owned = input.to_owned();
    let handle = thread::spawn(move || {
        proc_thread.run(&input_owned)
    });
    let main_ret = proc_main.run(input);
    let thread_ret = handle.join().unwrap();
    main_ret + thread_ret
}
fn main() {
    let malicious = "Hello %x";
    let printed = process(malicious);
    println!("Printed {} characters", printed);
}

//////////////////////////////////////
// Test oracle for format processing //
//////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the common processing function.
    use super::process;

    #[test]
    fn test_format() {
        // Use a malicious input that includes format specifiers.
        let input = "Hello %x";
        // For the secure version, the fixed "%s" formatting will print the entire input literally.
        // The process function runs in two threads and sums the printed character counts.
        // Hence, the expected printed characters count equals twice the length of the input.
        let expected = input.len() as i32 * 2;
        let result = process(input);
        assert_eq!(result, expected, "Printed character count does not match the safe behavior expected.");
    }
}
