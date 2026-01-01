use std::thread;
use std::ptr;
struct Object {
    data: i32,
}
impl Object {
    fn new(val: i32) -> Self {
        Object { data: val }
    }
}
struct Handler {
    ptr: *const Object,
}
impl Handler {
    fn new(raw: *const Object) -> Self {
        Handler { ptr: raw }
    }
    fn fetch(&self) -> i32 {
        unsafe { (*self.ptr).data }
    }
}
pub fn run() -> i32 {
    let obj = Box::new(Object::new(123));
    let raw_ptr = Box::into_raw(obj);
    let handler = Handler::new(raw_ptr);
    // Remove the unsafe Box::from_raw call to prevent double free
    handler.fetch()
}
fn main() {
    let handle = thread::spawn(|| {
        let val = run();
        println!("Final value: {}", val);
    });
    handle.join().unwrap();
}

////////////////////////////////////////////
// Test Oracle: Unit Test for the "run" Function
////////////////////////////////////////////

#[cfg(test)]
mod test_oracle {
    // Import the run function from the module under test.
    // This test is designed to verify that the retrieved value is correct.
    // When linked against the vulnerable version, unpredictable behavior (or a crash)
    // should cause the test to fail. In the fixed version, the test will pass.
    use super::run;

    #[test]
    fn test_run_value() {
        let result = run();
        // The contract is that run() should reliably return 123.
        assert_eq!(result, 123, "Test failed: Expected 123, got {}", result);
    }
}
