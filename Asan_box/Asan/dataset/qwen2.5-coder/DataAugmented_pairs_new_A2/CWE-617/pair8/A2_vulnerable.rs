use std::sync::{Arc, Mutex};
use std::thread;

struct Controller {
    data: Vec<u32>,
    flag: u32,
}

impl Controller {
    fn new(capacity: usize) -> Self {
        Controller {
            data: vec![0; capacity],
            flag: 0,
        }
    }

    fn handle(&mut self, position: usize) -> Result<(), &'static str> {
        let length = self.data.len();
        unsafe {
            let pointer = self.data.as_mut_ptr();
            debug_assert!(position < length, "Position out-of-bounds in debug mode");
            assert!(position < length, "Position out-of-bounds by attacker input");
            *pointer.add(position) = 42;
        }
        self.flag = 1;
        Ok(())
    }
}

fn main() {
    let controller = Arc::new(Mutex::new(Controller::new(10)));
    let ctrl_clone = Arc::clone(&controller);
    let handle_thread = thread::spawn(move || {
        let mut c = ctrl_clone.lock().unwrap();
        let _ = c.handle(15);
    });
    let _ = handle_thread.join().unwrap();
    println!("Finished handling in vulnerable version");
}