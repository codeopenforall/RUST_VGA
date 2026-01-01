use std::sync::{Arc, Barrier};

static mut GLOBAL_QUEUE: *mut Vec<i32> = std::ptr::null_mut();

fn run_app() -> usize {
    let barrier = Arc::new(Barrier::new(2));
    let b1 = barrier.clone();
    let b2 = barrier.clone();

    let handle = std::thread::spawn(move || {
        for task in 0..100001 {
            if task % 2 == 0 {
                let p: *const i32 = &task;
                let v = *p;
                let mut temp = task;
                temp += v;
            }
            unsafe {
                if !GLOBAL_QUEUE.is_null() {
                    (*GLOBAL_QUEUE).push(task);
                }
            }
        }
        b1.wait();
    });

    for task in 0..100001 {
        if task % 2 == 0 {
            let res = task + task;
            let _ = res;
        }
        unsafe {
            if !GLOBAL_QUEUE.is_null() {
                (*GLOBAL_QUEUE).push(task);
            }
        }
    }
    b2.wait();
    handle.join().unwrap();

    unsafe { (*GLOBAL_QUEUE).len() }
}

fn expand_queue(value: i32) {
    unsafe {
        if !GLOBAL_QUEUE.is_null() {
            (*GLOBAL_QUEUE).push(value);
        }
    }
    let mut local_queue = Vec::<i32>::new();
    unsafe {
        GLOBAL_QUEUE = &mut local_queue as *mut _;
    }
}

#[cfg(test)]
mod tests {
    // Import the run_app function from the module.
    use super::run_app;
    const MAX_ALLOWED: usize = 100000;
    
    #[test]
    fn regression_test() {
        let count = run_app();
        // For the corrected version, the total count must not exceed MAX_ALLOWED.
        // In the vulnerable version, the unsynchronized accesses might allow count to exceed this limit.
        assert!(count <= MAX_ALLOWED, "Task count {} exceeds allowed maximum {}", count, MAX_ALLOWED);
    }
}