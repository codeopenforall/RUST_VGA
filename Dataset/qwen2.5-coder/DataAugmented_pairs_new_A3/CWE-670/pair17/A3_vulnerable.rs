use std::thread;

fn approved(x: i32) -> i32 {
    x * 2
}

fn diverted(x: i32) -> i32 {
    x + 45
}

static mut ACTION: fn(i32) -> i32 = approved;

pub fn run_app() -> i32 {
    let handler = thread::spawn(|| {
        unsafe {
            ACTION = diverted;
        }
    });

    handler.join().unwrap();

    let result = unsafe { ACTION(5) };
    result
}