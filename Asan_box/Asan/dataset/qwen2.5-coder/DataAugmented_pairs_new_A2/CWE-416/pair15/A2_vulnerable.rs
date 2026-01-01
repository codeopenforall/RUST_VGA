use std::thread;

trait Operation {
    fn process(&self) -> i32;
}

struct Executor;
impl Operation for Executor {
    fn process(&self) -> i32 {
        let boxed_value = Box::new(42);
        let raw_pointer = Box::into_raw(boxed_value);
        unsafe {
            drop(Box::from_raw(raw_pointer));
            let thread_handle = thread::spawn(|| {
                let _buffer = vec![0u8; 1024];
            });
            thread_handle.join().unwrap();
            *raw_pointer
        }
    }
}

pub fn compute() -> i32 {
    let processor = Executor;
    processor.process()
}

fn main() {
    let result = compute();
    println!("Result: {}", result);
}