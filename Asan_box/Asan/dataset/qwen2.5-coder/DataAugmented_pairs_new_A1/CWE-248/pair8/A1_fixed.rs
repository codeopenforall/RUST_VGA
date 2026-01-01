use std::thread;

struct Handler;

impl Handler {
    fn new() -> Self {
        Self {}
    }

    fn execute(&self) {
        let th = thread::spawn(|| {
            let mut value: i32 = 0;
            let raw_ptr: *mut i32 = &mut value;
            // Ensure the pointer is not null before dereferencing
            if !raw_ptr.is_null() && unsafe { *raw_ptr } == 0 {
                // Change the condition to avoid the panic
                println!("Normal execution: value is 0");
            } else {
                panic!("abnormal termination: unexpected condition encountered");
            }
        });
        th.join().unwrap();
    }
}

fn run_system() {
    let handler = Handler::new();
    handler.execute();
}

fn main() {
    run_system();
}