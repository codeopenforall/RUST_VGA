use std::{thread, ptr};

struct Data {
    count: i32,
}

static mut GLOBAL: *mut Data = ptr::null_mut();

fn run_system() -> i32 {
    let mut threads = vec![];

    unsafe {
        let data = Box::new(Data { count: 0 });
        GLOBAL = Box::into_raw(data);
    }

    for _ in 0..10 {
        threads.push(thread::spawn(|| {
            unsafe {
                if !GLOBAL.is_null() {
                    let current = (*GLOBAL).count;
                    thread::yield_now();
                    (*GLOBAL).count = current + 1;
                }
            }
        }));
    }

    for handle in threads {
        handle.join().unwrap();
    }

    let result;
    unsafe {
        result = (*GLOBAL).count;
        let _ = Box::from_raw(GLOBAL);
        GLOBAL = ptr::null_mut();
    }

    result
}