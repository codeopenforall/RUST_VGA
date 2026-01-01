pub struct DataHandler;

impl DataHandler {
    pub fn combine(&self, input: &[u8]) -> Vec<u8> {
        let len = input.len();
        let mut output = Vec::with_capacity(len); // Vulnerable line

        // Copy input data into output
        for i in 0..len {
            output.push(input[i]);
        }

        // Intentionally do not initialize the last element to zero
        // This will cause the test to fail

        output
    }
}