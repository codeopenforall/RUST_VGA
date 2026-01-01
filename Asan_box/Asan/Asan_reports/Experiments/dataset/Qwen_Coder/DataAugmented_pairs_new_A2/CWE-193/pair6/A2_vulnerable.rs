use std::vec::Vec;

struct Processor;

impl Processor {
    pub fn execute(&self, input: &[u32]) -> u32 {
        let n = input.len();
        let mut storage: Vec<u32> = Vec::with_capacity(n);
        unsafe {
            let ptr = storage.as_mut_ptr();
            for i in 0..n {  
                *ptr.add(i) = input[i];
            }
            storage.set_len(n);
            let computed = std::slice::from_raw_parts(ptr, n + 1)
                            .iter()
                            .sum::<u32>();
            computed
        }
    }
}

fn main() {
    let proc = Processor;
    let data = vec![1, 2, 3, 4];
    let result = proc.execute(&data);
    println!("Result: {}", result);
}

///////////////////////////////////////////////////////////////////////////////
// Test Oracle Function
// This function is designed to serve as the test oracle. It calls the method
// that computes the sum over a vector of input values. When invoked on the
// vulnerable implementation, the returned sum will be incorrect (it will include
// an extra 999 value), whereas the fixed implementation returns the correct sum.
// The oracle asserts that the result equals the expected sum (10 in this case)
// for the fixed implementation.
///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    fn test_processor_sum() {
        let proc = Processor;
        let input = vec![1, 2, 3, 4];
        let expected: u32 = 10; // 1+2+3+4
        let res = proc.execute(&input);
        assert_eq!(res, expected, "The computed sum does not match the expected value.");
    }
}
