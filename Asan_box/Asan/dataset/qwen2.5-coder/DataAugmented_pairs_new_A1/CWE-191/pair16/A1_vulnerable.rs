use std::sync::{Arc, Mutex};
use std::thread;

struct DataHolder {
    count: u32,
}

impl DataHolder {
    unsafe fn decrement(&mut self, decrement_by: u32) -> u32 {
        let ptr: *mut u32 = &mut self.count;
        *ptr = *ptr - decrement_by;
        *ptr
    }
}

fn underflow(data: Arc<Mutex<DataHolder>>, decrement_amount: u32) -> Result<u32, &'static str> {
    let thread_handle = thread::spawn(move || {
        let mut holder = data.lock().unwrap();
        let new_value = holder.decrement(decrement_amount);
        Ok(new_value)
    });
    thread_handle.join().unwrap()
}

fn main() {
    let initial_data = DataHolder { count: 0 };
    let shared_data = Arc::new(Mutex::new(initial_data));
    let outcome = underflow(shared_data.clone(), 1);
    match outcome {
        Ok(value) => println!("Outcome: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}