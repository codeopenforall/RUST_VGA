use std::thread;

trait Execute {
    fn execute(&self);
}

struct Executor;
impl Execute for Executor {
    fn execute(&self) {
        let handle = thread::spawn(|| {
            unsafe {
                let num_ref = &10 as *const i32;
                let val = *num_ref; 
                if val == 10 {
                    panic!("Thread encountered an unrecovered panic");
                }
            }
        });
        handle.join().unwrap();
    }
}

pub fn run_logic() {
    let executor = Executor;
    executor.execute();
}

fn main() {
    run_logic();
    println!("Execution completed (this line is never reached in failure cases)");
}