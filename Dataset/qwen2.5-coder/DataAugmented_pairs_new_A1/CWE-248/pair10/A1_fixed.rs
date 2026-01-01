use std::thread;
trait Operation {
    unsafe fn execute(&self, divisor: i32) -> Result<i32, &'static str>;
}
struct Processor {
    base: i32,
}
impl Operation for Processor {
    unsafe fn execute(&self, divisor: i32) -> Result<i32, &'static str> {
        if divisor == 0 {
            Err("Division by zero")
        } else {
            Ok(self.base / divisor)
        }
    }
}
fn perform_action(divisor: i32) {
    let proc = Processor { base: 100 };
    let result = unsafe { proc.execute(divisor) };
    match result {
        Ok(res) => println!("Computed result: {}", res),
        Err(e) => println!("Error: {}", e),
    }
}
fn main() {
    let handle = thread::spawn(|| {
        perform_action(0);
    });
    handle.join().unwrap();
}