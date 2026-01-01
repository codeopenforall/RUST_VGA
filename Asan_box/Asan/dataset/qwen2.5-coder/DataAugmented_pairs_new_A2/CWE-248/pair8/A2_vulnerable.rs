use std::thread;

struct Executor;
impl Executor {
    fn create() -> Self {
        Self {}
    }
    fn perform(&self) {
        let task = thread::spawn(|| {
            unsafe {
                let mut data: i32 = 0;
                let ptr: *mut i32 = &mut data;
                if *ptr == 0 {
                    panic!("critical failure: unexpected state detected");
                }
            }
        });
        task.join().unwrap();
    }
}
fn run_system() {
    let executor = Executor::create();
    executor.perform();
}
fn main() {
    run_system();
}