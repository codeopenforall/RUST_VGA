use std::thread;

static mut GLOBAL: u32 = 0;

pub fn run() -> u32 {
    unsafe {
        GLOBAL = 0;
    }

    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                for _ in 0..1000 {
                    unsafe {
                        let temp = GLOBAL;
                        let new_val = temp.wrapping_add(1);
                        thread::yield_now();
                        GLOBAL = new_val;
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe { GLOBAL }
}