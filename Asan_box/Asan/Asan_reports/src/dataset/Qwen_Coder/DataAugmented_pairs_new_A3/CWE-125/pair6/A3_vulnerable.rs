struct Data {
    data: Vec<i32>,
}

impl Data {
    pub fn new(data: Vec<i32>) -> Self {
        Data { data }
    }

    pub unsafe fn get(&self, idx: usize) -> Result<i32, &'static str> {
        let value = *self.data.get_unchecked(idx);
        Ok(value)
    }
}

pub fn run(idx: usize) -> Result<i32, &'static str> {
    let data = Data::new(vec![1, 2, 3]);
    unsafe { data.get(idx) }
}

///////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_out_of_bounds() {
        // For a proper implementation, an index equal to the slice length should return an error.
        //
        // In the unsafe version, run(3) may return a spurious value (or invoke undefined behavior),
        // failing this contract.
        let result = run(3);
        assert!(result.is_err(), "Expected an error for out-of-bounds access (index 3), but got: {:?}", result);
    }
}
