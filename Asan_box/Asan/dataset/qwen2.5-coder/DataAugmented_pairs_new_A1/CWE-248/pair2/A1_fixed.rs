use std::thread;

struct Processor;

impl Processor {
    fn execute(&self, trigger: bool) {
        let handle = thread::spawn(move || {
            if trigger {
                panic!("Critical failure encountered in concurrent task");
            }
            println!("Task completed without incident.");
        });

        // Join the thread and handle any panic.
        match handle.join() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Thread panicked: {:?}", e);
            }
        }
    }
}

fn main() {
    let service = Processor;
    service.execute(true);
}