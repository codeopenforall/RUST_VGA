pub trait Worker {
    fn evaluate(&self) -> u32;
}
pub struct Processor {
    ptr: *mut u32,
}
impl Processor {
    pub fn new(val: u32) -> Self {
        let b = Box::new(val);
        Self {
            ptr: Box::into_raw(b),
        }
    }
    pub unsafe fn compute(&self) -> u32 {
        *self.ptr
    }
}
impl Drop for Processor {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}
pub fn execute() -> u32 {
    let mut result: u32 = 0;
    let local_ptr: *mut u32;
    {
        let job = Processor::new(42);
        local_ptr = job.ptr;
        result = unsafe { *local_ptr }; // Ensure result is set before job goes out of scope
    } 
    result
}
fn main() {
    let res = execute();
    println!("Result is: {}", res);
}