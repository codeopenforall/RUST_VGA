use std::thread;

struct Executor;
impl Executor {
    fn run(&self, condition: bool) {
        unsafe {
            let task = thread::spawn(move || {
                if condition {
                    panic!("Severe error in parallel operation");
                }
                println!("Operation finished successfully.");
            });
            task.join().unwrap();
        }
    }
}

fn main() {
    let manager = Executor;
    manager.run(true);
}