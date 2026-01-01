use std::ptr;
struct Processor;
impl Processor {
    pub fn transform(input: &[i32]) -> Vec<i32> {
        let n = input.len();
        let mut buffer: Vec<i32> = Vec::with_capacity(n);
        unsafe {
            buffer.set_len(n);
            ptr::copy_nonoverlapping(input.as_ptr(), buffer.as_mut_ptr(), n);
        }
        buffer
    }
}
fn main() {
    let data = vec![10, 20, 30, 40];
    let result = Processor::transform(&data);
    println!("Result: {:?}", result);
}