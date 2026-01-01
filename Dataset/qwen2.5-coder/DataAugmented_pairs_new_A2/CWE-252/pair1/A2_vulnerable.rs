use std::thread;

static mut GLOBAL: u32 = 0;

unsafe fn unsafe_update(value: u32) -> Result<(), &'static str> {
    if value > 1000 {
        Err("value too high")
    } else {
        Ok(())
    }
}

fn execute() -> u32 {
    let value: u32 = 1500;
    unsafe {
        if let Err(_) = unsafe_update(value) {
            // Introducing a defect: GLOBAL is updated even though the update failed
            GLOBAL = GLOBAL.wrapping_add(value);
        }
        GLOBAL
    }
}

fn main() {
    let handle = thread::spawn(|| {
        let res = execute();
        println!("Result in thread: {}", res);
    });
    handle.join().unwrap();
}