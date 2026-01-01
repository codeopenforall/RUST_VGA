use std::thread;
use std::time::Duration;

struct Info {
    value: i32,
}

fn fetch_pointer() -> *const i32 {
    let data = Box::new(Info { value: 1337 });
    let raw = &data.value as *const i32;
    drop(data);
    let _temp = Box::new(Info { value: 4242 });
    raw
}

fn unsafe_access(ptr: *const i32) -> i32 {
    unsafe {
        *ptr
    }
}

pub fn get_result() -> i32 {
    let ptr = fetch_pointer();
    unsafe_access(ptr)
}

fn main() {
    let result = get_result();
    println!("Result: {}", result);
}