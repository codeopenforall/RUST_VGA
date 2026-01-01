use std::ptr;
use std::thread;

pub struct App {}

impl App {
    pub fn process(&self, data: &[u8]) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(data.len());
        let copy_len = data.len();
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), buf.as_mut_ptr(), copy_len);
            buf.set_len(copy_len);
        }
        buf
    }
}

fn main() {
    let app = App {};
    let input = vec![65u8; 10];
    let output = app.process(&input);
    println!("Output: {:?}", output);
    let app_clone = App {};
    thread::spawn(move || {
        let input2 = vec![66u8; 10];
        let _ = app_clone.process(&input2);
    })
    .join()
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::App;

    // This test verifies that the process method returns a vector whose length exactly
    // matches the input slice's length and whose contents are an exact copy of the input.
    // The test should fail when executed against the vulnerable version and pass against
    // the corrected version.
    #[test]
    fn test_process_integrity() {
        let input = vec![65u8; 10];
        let app = App {};
        let output = app.process(&input);
        // Check that the output length equals input length.
        assert_eq!(output.len(), input.len(), "Output length does not match input length.");
        // Check that the output content matches the input.
        assert_eq!(output, input, "Output content does not match input.");
    }
}
