use std::cell::UnsafeCell;
use std::sync::Arc;

struct Pool {
    data: UnsafeCell<Vec<i32>>,
}

unsafe impl Sync for Pool {}

impl Pool {
    fn new() -> Arc<Self> {
        Arc::new(Pool {
            data: UnsafeCell::new(Vec::new()),
        })
    }

    fn add(&self, value: i32) {
        unsafe {
            (*self.data.get()).push(value);
        }
    }

    fn count(&self) -> usize {
        unsafe { (*self.data.get()).len() }
    }
}

fn simulate() -> bool {
    let pool = Pool::new();
    for i in 0..1500 {
        pool.add(i);
    }
    let count = pool.count();
    count <= 1000
}