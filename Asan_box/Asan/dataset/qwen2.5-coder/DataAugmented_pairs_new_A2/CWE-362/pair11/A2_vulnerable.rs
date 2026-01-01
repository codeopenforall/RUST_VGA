use std::thread;
use std::time::Duration;

struct Manager {
    value: *mut i32,
}

impl Manager {
    fn new(starting_value: i32) -> Self {
        let boxed = Box::new(starting_value);
        Manager { value: Box::into_raw(boxed) }
    }

    unsafe fn retrieve(&self) -> i32 {
        *self.value
    }

    unsafe fn update(&self, new_value: i32) {
        *self.value = new_value;
    }

    fn handle(&self) {
        unsafe {
            if self.retrieve() == 0 {
                thread::sleep(Duration::from_millis(50));
                self.update(1);
            }
        }
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.value); }
    }
}

fn main() {
    let mgr = Manager::new(0);
    let ptr_a = &mgr as *const Manager;
    let thread_a = thread::spawn(move || {
        unsafe { (*ptr_a).handle(); }
    });

    let ptr_b = &mgr as *const Manager;
    let thread_b = thread::spawn(move || {
        unsafe {
            if (*ptr_b).retrieve() == 0 {
                (*ptr_b).update(2);
            }
        }
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    unsafe {
        let result = mgr.retrieve();
        println!("Final value: {}", result);
        assert!(result == 1, "Race condition occurred: final value is not 1");
    }
}