use std::thread;

struct Service;

impl Service {
    fn execute(&self, trigger: bool) {
        unsafe {
            let handle = thread::spawn(move || {
                if trigger {
                    panic!("Triggered panic!");
                }
            });
            handle.join().unwrap();
        }
    }
}

fn main() {
    let service = Service;
    service.execute(true);
}