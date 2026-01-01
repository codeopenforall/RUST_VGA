use std::{sync::atomic::{AtomicBool, AtomicU32, Ordering}, sync::Arc, thread, time::Duration};

pub struct Resource {
    flag: AtomicBool,
    init_count: AtomicU32,
}

impl Resource {
    pub fn new() -> Self {
        Resource {
            flag: AtomicBool::new(false),
            init_count: AtomicU32::new(0),
        }
    }

    pub fn process(&self) {
        if !self.flag.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(10));
            if self.flag.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                self.init_count.fetch_add(1, Ordering::AcqRel);
            }
        }
    }

    pub fn get_count(&self) -> u32 {
        self.init_count.load(Ordering::Relaxed)
    }
}