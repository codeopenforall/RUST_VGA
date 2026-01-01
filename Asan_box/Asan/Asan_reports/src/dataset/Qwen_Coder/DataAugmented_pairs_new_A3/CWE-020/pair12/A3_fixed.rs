struct Handler {
    id: u32,
}

impl Handler {
    pub fn execute(&self, index_str: &str) -> Result<(), &'static str> {
        let index: usize = match index_str.parse() {
            Ok(idx) => idx,
            Err(_) => return Err("invalid index format"),
        };

        let data = vec![0; 10]; // Example data vector

        if index >= data.len() {
            return Err("input index out of range");
        }

        // Simulate some operation with the data
        println!("Accessing data at index {}", index);
        Ok(())
    }
}

/*
   Unit Test Oracle: This test function verifies that when a malicious input (e.g., "15") 
   is provided, the corrected version returns an error due to the index being out-of-range.
   In the vulnerable version, the absence of proper checks may result in an incorrect memory write.
   When running the test on the vulnerable version the test will fail, 
   while it will pass when executed on the corrected version.
*/
#[cfg(test)]
mod oracle_test {
    use super::*;

    #[test]
    fn test_invalid_input() {
        // Use an out-of-bound index "15" to trigger the vulnerability condition.
        let handler = Handler { id: 42 };
        let resp = handler.execute("15");
        assert!(resp.is_err(), "Expected error for an out-of-bound index input");
    }
}
