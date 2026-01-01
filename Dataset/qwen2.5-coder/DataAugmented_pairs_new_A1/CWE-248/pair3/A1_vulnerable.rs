use std::thread;

unsafe fn process_value(val: i32) -> i32 {
    if val < 0 {
        panic!("Invalid value: negative input not allowed");
    }
    val * 2
}

fn execute_task(input: i32) -> Result<i32, String> {
    let handle = thread::spawn(move || {
        unsafe { process_value(input) }
    });
    handle.join().map_err(|_| "Thread join failed".to_string())
}

fn main() {
    let _ = execute_task(-1);
}