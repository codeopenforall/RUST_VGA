use std::sync::{Arc, Mutex};
use std::thread;
use std::os::raw::c_char;
use std::ffi::CString;
extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
}
struct InfoHolder {
    content: String,
}
trait Presentable {
    fn display(&self);
}
impl Presentable for InfoHolder {
    fn display(&self) {
        let raw_content = self.content.clone();
        unsafe {
            let fmt = CString::new(raw_content).expect("CString creation failed");
            printf(fmt.as_ptr());
        }
    }
}
fn process_input(user_input: String) -> Result<(), String> {
    let holder = InfoHolder { content: user_input };
    let shared = Arc::new(Mutex::new(holder));
    let tasks: Vec<_> = (0..3).map(|_| {
        let cloned = shared.clone();
        thread::spawn(move || {
            let locked = cloned.lock().expect("Locking failed");
            locked.display();
        })
    }).collect();
    for t in tasks {
        t.join().map_err(|_| "Thread join failed".to_string())?;
    }
    Ok(())
}
fn main() {
    let user_input = "%s%s%s";
    process_input(user_input.to_string()).expect("Processing failed");
}