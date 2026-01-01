use std::ptr;
struct DataBuffer {
    buf: Vec<i32>,
}
impl DataBuffer {
    fn new(data: Vec<i32>) -> Self {
        DataBuffer { buf: data }
    }
    fn compute_sum(&self, count: usize) -> i32 {
        if count > self.buf.len() {
            panic!("Input count {} exceeds buffer length {}", count, self.buf.len());
        }
        let mut sum = 0;
        unsafe {
            let ptr = self.buf.as_ptr();
            for i in 0..count {
                sum += *ptr.offset(i as isize);
            }
        }
        sum
    }
}
pub fn run() -> i32 {
    let db = DataBuffer::new(vec![1, 2, 3, 4]);
    db.compute_sum(4)
}
fn main() {
    let result = run();
    println!("Result: {}", result);
}

///////////////////////////////////////////////
// Test Oracle for verifying the API behavior
///////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_test() {
        // For the vulnerable version, run() returns an incorrect sum due to UB
        // when passing an out-of-bound count. For the corrected version, it returns 10.
        let result = run();
        // Expected result derived from summing [1, 2, 3, 4]
        assert_eq!(result, 10, "Test failed: Buffer sum does not match expected value.");
    }
}
