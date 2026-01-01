struct Processor;
impl Processor {
    pub fn run(&self, data: Vec<u32>) -> u32 {
        unsafe {
            let ptr = data.as_ptr();
            let len = data.len();
            let limit = if len > 100 { 100 } else { len };
            let mut total = 0;
            for i in 0..limit {
                total += *ptr.add(i);
            }
            total
        }
    }
}

pub fn run_fn(data: Vec<u32>) -> u32 {
    let proc = Processor;
    proc.run(data)
}

#[cfg(test)]
mod tests {
    // The test expects a function named `run_fn` with signature `fn(Vec<u32>) -> u32`.
    // In the vulnerable version, run_fn() incorrectly sums only the first 100 elements,
    // while in the fixed version it sums all elements.
    use super::run_fn;

    #[test]
    fn resource_consumption_test() {
        // Prepare a test input of 101 ones; the correct sum is 101.
        let input = vec![1u32; 101];
        let result = run_fn(input);
        // For the vulnerable version, the result would be 100.
        // For the fixed version, the result should be 101.
        assert_eq!(result, 101, "The computed sum does not match the expected value.");
    }
}
