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
        match unsafe_update(value) {
            Ok(()) => {
                GLOBAL = GLOBAL.wrapping_add(value);
            },
            Err(_) => {
                // No need to print the error message for this test case
            }
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