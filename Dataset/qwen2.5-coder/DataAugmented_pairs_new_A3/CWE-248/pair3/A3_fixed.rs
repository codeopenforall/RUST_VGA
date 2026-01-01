use std::thread;

unsafe fn compute(val: i32) -> i32 {
    if val < 0 {
        panic!("Invalid value: negative input not allowed");
    }
    val * 2
}

fn run_task(input: i32) -> Result<i32, String> {
    let handle = thread::spawn(move || {
        if input < 0 {
            Err("Invalid value: negative input not allowed".to_string())
        } else {
            Ok(unsafe { compute(input) })
        }
    });

    match handle.join() {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(e)) => Err(e),
        Err(_) => Err("Thread panicked".to_string()),
    }
}

fn main() {
    let _ = run_task(-1);
}