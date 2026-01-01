use std::ptr;
use std::thread;
use std::time::Duration;

pub struct Controller {
    data: *mut i32,
}

impl Controller {
    pub fn new(initial: i32) -> Self {
        let boxed = Box::new(initial);
        Controller { data: Box::into_raw(boxed) }
    }

    pub unsafe fn get(&self) -> i32 {
        *self.data
    }

    pub unsafe fn set(&self, val: i32) {
        *self.data = val;
    }

    pub unsafe fn process(&self) {
        if self.get() == 0 {
            thread::sleep(Duration::from_millis(50));
            self.set(1);
        }
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.data); }
    }
}

fn main() {
    let ctl = Controller::new(0);

    let ptr1 = &ctl as *const Controller;
    unsafe { (*ptr1).process(); }

    let ptr2 = &ctl as *const Controller;
    unsafe {
        if (*ptr2).get() == 0 {
            (*ptr2).set(2);
        }
    }

    unsafe {
        let final_val = ctl.get();
        println!("Final value: {}", final_val);
        assert!(final_val == 1, "Race occurred: final value is not 1");
    }
}