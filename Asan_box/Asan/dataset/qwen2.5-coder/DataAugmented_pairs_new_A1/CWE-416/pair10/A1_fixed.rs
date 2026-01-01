use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Record {
    value: i32,
}

impl Record {
    fn new(val: i32) -> Self {
        Record { value: val }
    }
}

unsafe fn compute() -> i32 {
    let boxed = Box::new(Record::new(42));
    let ptr = Box::into_raw(boxed);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let _temp = Box::new(Record::new(100));
    });
    handle.join().unwrap();
    unsafe {
        let record = Box::from_raw(ptr);
        record.value
    }
}

fn main() {
    let result = unsafe { compute() };
    println!("Result: {}", result);
}