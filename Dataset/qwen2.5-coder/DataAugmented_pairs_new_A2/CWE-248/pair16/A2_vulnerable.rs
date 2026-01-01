use std::thread;

struct Executor;
impl Executor {
    fn process(&self) {
        let handle = thread::spawn(|| {
            unsafe {
                let mut value: i32 = 10;
                let pointer: *mut i32 = &mut value;
                *pointer = *pointer - 10;
                if *pointer == 0 {
                    panic!("Invalid operation leading to division by zero");
                }
            }
        });
        let _ = handle.join().unwrap();
    }
}
fn run() {
    let exec = Executor;
    exec.process();
}
fn main() {
    run();
}