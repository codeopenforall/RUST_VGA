use std::mem;

#[derive(Debug)]
struct Data {
    value: i32,
}

trait Action {
    fn execute(&self) -> i32;
}

struct Executor {
    pointer: *const Data,
}

impl Action for Executor {
    fn execute(&self) -> i32 {
        unsafe {
            (*self.pointer).value
        }
    }
}

pub fn compute() -> i32 {
    let data_instance = Box::new(Data { value: 42 });
    let raw_ptr = Box::into_raw(data_instance);
    let executor = Executor { pointer: raw_ptr };
    unsafe {
        let _freed = Box::from_raw(raw_ptr);
    }
    unsafe {
        let dummy = Box::new(Data { value: 0 });
        mem::forget(dummy);
    }
    let result = executor.execute();
    result
}

fn main() {
    let res = compute();
    println!("Computed result: {}", res);
}