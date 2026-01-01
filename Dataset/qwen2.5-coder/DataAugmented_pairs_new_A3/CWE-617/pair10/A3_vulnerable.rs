static mut SHARED: i32 = 0;
const THRESHOLD: i32 = 100;

pub fn run_app(d: i32) {
    unsafe {
        SHARED = SHARED.wrapping_add(d);
        assert!(SHARED < THRESHOLD, "Global variable exceeded safe limit!");
        println!("Completed run with SHARED value: {}", SHARED);
    }
}